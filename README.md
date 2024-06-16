# RAW
## Rust Audio Workstation
What is it?

Complex exponential reverb synthesis

## Other things to think about
Phaser

Not sure why I cant have just a reverb unit with no phase

The Nonlinear is pretty good for getting a big chonky cooked thing
needs to be able to make like entire patch or like coupling matrix. Coupling nonlinear operator too. or like matrix for distortion strength or whatever
longer waveguide probs good
Maybe we set up N waveguides including if it has nonlinear, what its coefficients are

coeffs of transmission and reflection any constraint?

Wanting to introduce second impedance

We dont have to do the matrix we can have a block of code instead

Consider it making an instrument

The instrument would need a frequency input which is used as the phase on the first thing
And the envelope input which is used as the magnitude.
So its complex exponentials all the way down sir.

Do I make some fkn dyn series and dyn parallel
Parallel hasnt actually been good though it can in theory make a phaser


Whats the waveguides in a drum

Part of it is the air between it and your ear probably
How to get slow attack, by default its instant. fkn needs delay


then theres if the whole delay line has coefficients a la complex fir. complex fir with feedback. or iir


so yea could easily expand with longer delays ain separate coefficients, then it basically is waveguides

ideally combining multiple modes of an instrument harmonic and inharmonic

its like fm, i can see how you would do that too
can shape the envelope etc easy

cross coupling will be good: matrix floor of 0.1

i wonder if it makes sense to do a finite element simulation where phase of wave is changed in between each of the things.
might relate to what is the speed of the wave in that medium.

if its back n forth on a string should it have 2 ways ring buffer
it sounds real fucked with the long delay cause it starts as 0

i reckon build structures out of box dyn traits. willl be sick


does a bunch of phasors degenerate to one phasor or no


what have we learned from this, first one was still kind of the best lol.

what if we just convolved a bunch of LTI signals including maybe some complex ones.

Important to remember i think for instrument that its a complex exponential, frequency and magnitude
maybe really a linear function of time held sorta thing, into adsr
thats one way, or it can be like ones while held, feedback.

the idea of conserving energy seems kinda important tbh.
the idea was to kind of simulate driving a system and witnessing its mode collapse and push modes

still yet to rly try some of that other shit like the 2 ways on a string one

I kinda reckon no  traits
no nothing
concrete shit all the way