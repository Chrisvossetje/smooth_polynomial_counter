This repos purpose is counting the amount of smooth homogeneous polynomials of degree n in the P^2 space over a finite field F_q.

Because of the scientific nature of this, this code isn't meant to be generally reused.

We take a polynomial and we check whether or not it is singular in any k_i field extension. You can also run this code faster if you check all isomorphism classes of polynomials. Because translating a polynomial under PGL_3(F_q) is the same as evaluating in different points.

At this point only F2 and F3 are implemented, because the time complexity rises incredibly quickly with higher q.
However the biggest problem is not per se the amount of points to check. But the generation of all isomorphism classes.
The code for the isomorphism classes can be found [here](https://github.com/Jake-O-Dev/isopoly_generation)

Developed by,  
Jacco Hijmans and Chris Vos  
Assisted by,  
Prof. Dr. C.F. Faber at Utrecht University