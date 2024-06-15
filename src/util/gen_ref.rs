use std::marker::PhantomData;

#[derive(Debug)]
pub struct GenRef<T> {
    idx: usize,
    gen: usize,
    _marker: PhantomData<T>,
}

impl<T> Clone for GenRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for GenRef<T> {}

impl<T> Default for GenRef<T> {
    fn default() -> Self {
        GenRef {
            idx: 0,
            gen: usize::MAX,
            _marker: PhantomData,
        }
    }
}

impl<T> PartialEq for GenRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.gen == other.gen
    }
}

impl<T: Clone> GenRef<T> {
    pub fn deref<'a>(&self, arena: &'a GenArena<T>) -> Option<&'a T> {
        arena.borrow(self)
    }
    pub fn deref_mut<'a>(&self, arena: &'a mut GenArena<T>) -> Option<&'a mut T> {
        arena.borrow_mut(self)
    }
    pub fn free<'a>(&self, arena: &'a mut GenArena<T>) {
        if let Some(bucket) = arena.arena.get_mut(self.idx) {
            if bucket.generation == self.gen && !bucket.free {
                bucket.free = true;
            }
        }
    }

    // change arenas
    pub fn change_arenas(&mut self, from: &GenArena<T>, to: &mut GenArena<T>) {
        *self = self
            .deref(from)
            .cloned()
            .map(|x| to.alloc(x))
            .unwrap_or_default();
    }
}

pub struct GenBucket<T> {
    pub item: Option<T>,
    pub free: bool,
    pub generation: usize,
}

impl<T> GenBucket<T> {
    fn new(item: T) -> Self {
        GenBucket {
            item: Some(item),
            free: false,
            generation: 0,
        }
    }
}

pub struct GenArena<T> {
    pub arena: Vec<GenBucket<T>>,
}

impl<T> GenArena<T> {
    pub fn new() -> Self {
        GenArena { arena: Vec::new() }
    }
    pub fn borrow<'a>(&'a self, gen_ref: &GenRef<T>) -> Option<&'a T> {
        self.arena.get(gen_ref.idx).and_then(|bucket| {
            bucket
                .item
                .as_ref()
                .filter(|_| !bucket.free && bucket.generation == gen_ref.gen)
        })
    }
    pub fn borrow_mut<'a>(&'a mut self, gen_ref: &GenRef<T>) -> Option<&'a mut T> {
        self.arena.get_mut(gen_ref.idx).and_then(|bucket| {
            bucket
                .item
                .as_mut()
                .filter(|_| !bucket.free && bucket.generation == gen_ref.gen)
        })
    }
    pub fn borrow_2<'a>(
        &'a self,
        gen_ref1: &GenRef<T>,
        gen_ref2: &GenRef<T>,
    ) -> Option<(&'a T, &'a T)> {
        let item1 = self.borrow(gen_ref1)?;
        let item2 = self.borrow(gen_ref2)?;
        Some((item1, item2))
    }
    pub fn borrow_2_mut<'a>(
        &'a mut self,
        gen_ref1: &GenRef<T>,
        gen_ref2: &GenRef<T>,
    ) -> Option<(&'a mut T, &'a mut T)> {
        let ptr = self.arena.as_mut_ptr();

        unsafe {
            let bucket_ptr1 = ptr.add(gen_ref1.idx);
            let bucket_ptr2 = ptr.add(gen_ref2.idx);

            let bucket1 = &mut *bucket_ptr1;
            let bucket2 = &mut *bucket_ptr2;

            if bucket1.generation == gen_ref1.gen
                && !bucket1.free
                && bucket2.generation == gen_ref2.gen
                && !bucket2.free
            {
                let item1 = bucket1.item.as_mut()?;
                let item2 = bucket2.item.as_mut()?;
                Some((item1, item2))
            } else {
                None
            }
        }
    }
    pub fn borrow_many<'a>(&'a self, gen_refs: &'a [GenRef<T>]) -> Option<Vec<&'a T>> {
        let mut borrowed_items = Vec::with_capacity(gen_refs.len());

        for gen_ref in gen_refs {
            if let Some(item) = self.borrow(gen_ref) {
                borrowed_items.push(item);
            } else {
                // If any item cannot be borrowed, return None
                return None;
            }
        }
        Some(borrowed_items)
    }
    pub fn borrow_many_mut<'a>(&'a mut self, gen_refs: &'a [GenRef<T>]) -> Option<Vec<&'a mut T>> {
        let mut borrowed_items = Vec::with_capacity(gen_refs.len());

        // Get raw pointer to the vector's data
        let ptr = self.arena.as_mut_ptr();

        for gen_ref in gen_refs {
            unsafe {
                // Get a raw pointer to the bucket corresponding to the index
                let bucket_ptr = ptr.add(gen_ref.idx);

                // Convert the raw pointer to a mutable reference to GenBucket<T>
                let bucket = &mut *bucket_ptr;

                // Check if the generation matches and the bucket is not free
                if bucket.generation == gen_ref.gen && !bucket.free {
                    // Get a mutable reference to the item inside the bucket
                    if let Some(item) = bucket.item.as_mut() {
                        // Push the mutable reference to the item into the borrowed_items vector
                        borrowed_items.push(item);
                    } else {
                        // If the item is None, return None
                        return None;
                    }
                } else {
                    // If generation does not match or bucket is free, return None
                    return None;
                }
            }
        }
        Some(borrowed_items)
    }

    pub fn free(&mut self, gen_ref: &GenRef<T>) {
        if let Some(bucket) = self.arena.get_mut(gen_ref.idx) {
            if bucket.generation == gen_ref.gen {
                bucket.free = true;
            }
        }
    }
    pub fn alloc(&mut self, item: T) -> GenRef<T> {
        for (idx, bucket) in self.arena.iter_mut().enumerate() {
            if bucket.free {
                let old_generation = bucket.generation;
                bucket.item = Some(item);
                bucket.free = false;
                bucket.generation = old_generation + 1;
                return GenRef {
                    idx,
                    gen: bucket.generation,
                    _marker: PhantomData,
                };
            }
        }

        let idx = self.arena.len();
        self.arena.push(GenBucket::new(item));

        GenRef {
            idx,
            gen: 0,
            _marker: PhantomData,
        }
    }
}
