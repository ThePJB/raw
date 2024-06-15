use crate::util::*;

// i wonder if the retarded one supports upsampling

fn dist_fn(p1: IVec2, p2: IVec2) -> f32 {
    let u = p1 - p2;
    (u.dot(&u) as f32).sqrt()
}

fn spread(distance: &mut Texture<f32>, nearest: &mut Texture<IVec2>, p1: IVec2, p2: IVec2) {
    if p1.x < 0 || p1.y < 0 || p2.x < 0 || p2.y < 0 { return; }
    let p1_nearest = nearest.sample(p1);
    if p1_nearest.x < 0 || p1_nearest.y < 0 { return; }
    let dp1 = dist_fn(p2, p1_nearest);
    if dp1 < distance.sample(p2) {
        distance.set(p2, dp1);
        nearest.set(p2, p1_nearest);
    }
}

pub fn sdf1(in_stencil: Texture<bool>) -> Texture<f32> {
    let wh = in_stencil.wh;
    let mut nearest = Texture::<IVec2>::new(wh, ivec2(-1, -1));
    let mut distance = Texture::<f32>::new(wh, f32::INFINITY);

    // stuff interior of that has negative distance
    // stuff outside of that has positive distance
    // spread(p1, p2) -> try and set p2s distance to p1s if its shorter. reducing distance
    // pretty sure we update with shorter distances yea

    // initialize boundary
    for i in 0..wh.x {
        for j in 0..wh.y {
            let p = ivec2(i,j);
            let neighbours = [ivec2(i-1,j), ivec2(i+1,j), ivec2(i,j-1), ivec2(i,j+1)];
            if in_stencil.sample(p) {
                if neighbours.iter().any(|n| {
                    let s = in_stencil.sample_checked(*n);
                    s.is_none() || s.unwrap() == false
                }) {
                    distance.set(p, 0.0);
                    nearest.set(p, p);
                }
            }
        }
    }
    
    for _ in 0..4 {
        for j in 0..wh.y {

            for i in 0..wh.x-1 {
                spread(&mut distance, &mut nearest, ivec2(i,j), ivec2(i+1, j));
            }
            for i in (1..wh.x).rev() {
                spread(&mut distance, &mut nearest, ivec2(i,j), ivec2(i-1, j));
            }
        }
        for i in 0..wh.x {
            for j in 0..wh.y-1 {
                spread(&mut distance, &mut nearest, ivec2(i,j), ivec2(i, j+1));
            }
            for j in (1..wh.y).rev() {
                spread(&mut distance, &mut nearest, ivec2(i,j), ivec2(i, j-1));
            }
        }
    }


    // spread rows +x 
    distance
}

impl Texture<f32> {
    pub fn normalize(&mut self) {
        let mut max = 0.0f32;
        for i in 0..self.wh.x {
            for j in 0..self.wh.y {
                max = max.max(self.sample(ivec2(i,j))) ;
            }
        }
        for i in 0..self.wh.x {
            for j in 0..self.wh.y {
                let p = ivec2(i,j);
                self.set(p, self.sample(p)/max);
            }
        }
    }
}


#[test]
fn test_sdf() {
    let wh = ivec2(256,256);
    let mut t: Texture<bool> = Texture::new(wh, false);
    t.draw_circle(ivec2(164,164), 32, true);
    t.draw_circle(ivec2(32,32), 4, true);
    t.draw_circle(ivec2(32,32), 4, true);
    let f_stencil = |s| {
        if s {
            [255, 255, 255, 255]
        } else {
            [0, 0, 0, 255]
        }
    };
    t.save(std::path::Path::new("./sdf_test_in.png"), f_stencil);

    let tstart = std::time::Instant::now();
    let mut sd = sdf1(t);
    let taken = std::time::Instant::now().duration_since(tstart).as_secs_f32();
    sd.normalize();
    let f = |d| {
        let v = (d * 255.0) as u8;
        [v, v, v, 255]
    };
    println!("making sdf for {:?} took {:.3}ms", wh, taken * 1000.0);
    sd.save(std::path::Path::new("./sdf_test.png"), f)
}


// i wonder if this faster than my bespoke sdf thing, i thought that was pretty clever bruh
// need modification to store source pixel as well, so it actually outputs a Texture<Vec3>, looks interesting I bet
// i wonder if based upsampling too or maybe JFA is kinda based upsampling

// hehe if we could make a iterator of ivec2s based on wh.row_major_iter

//lmao what about a monte carlo one where it just checks randomly tho, probably not bad
// could also just look up like the boundary ones only tbh. eg its O(num boundary px) ish each time
// or also spread based on locality like nearby boundary ones

// 4con vs diag 4con ie 8con-4con

// also need inside part obvs

// yea get some big jfa jumps in there, pretty easy to convert to jfa actually

// is it not the case that if you went in exact order of how far the distances were that it would correctly do it in minimal work. but that itself takes work.

// in all honestly its also fine for the distance field to be slightly bigger, or even manhattan
// just cant be smaller, for the raymarching.

// we can minus the distance on output easily to get negative, and we can easily combine with the uvs or whatever to get point as well
// usable for collisions etc!. or is it the other way, distance can be smaller for raymarching actually yeah. or not bigger by a known amt so we can apply that as multiplier.

// tho off screen enemies will clip, sus
// it also kinda is static tho this is quite dynamic, good for mining etc.

// hey its got applications in generation as well for nice rings etc. but i use implicit functions too. But you can warp and then do this.
// we gonna do some wolverson djikstra shit maybe for the dungeons too. lets see xd

// maybe we do like simple levels to the staff components as well that doubles effectiveness etc. tiers.