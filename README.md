# Spidonacci

A Rust program that calculates the biggest Fibonacci number that it can in a single second, for the following 5 methods:

## Naïve implementation
Where the Fibonacci numbers are calculated using the recurrence $F_n = F_{n-1} + F_{n-2}$.
## Matrix exponentiation
Matrix exponentiation, using *binary exponentiation* and the fact that:
$$
\begin{pmatrix}
F_{n+1} & F_{n} \\
F_{n} & F_{n-1}
\end{pmatrix}
= \begin{pmatrix}
1 & 1 \\
1 & 0
\end{pmatrix}^n,
$$
for all $n\geq 0$. We can exploit that the matrices are symmetric and only use have to calculate the upper triangular part (or the lower triangular part).

## Strassen like matrix exponentiation
Matrix multiplication that uses only 4 multiplications instead of 5 as in the other part at the cost of having to do 6 additions compared to 3.

## Closed form formula
This method uses the Binet formula for the Fibonacci numbers:
$$
F_n =\frac{\varphi^n -\psi^n}{\varphi-\psi}= \frac{(1+\sqrt{5})^n-(1-\sqrt{5})^n}{2^n\sqrt{5}},
$$
where $\varphi = \frac{1+\sqrt 5}{2}$ is the golden ratio and $\psi = 1- \varphi$. Now we can compute this efficiently using the fact that if $$(a+b\sqrt 5)^n = c + d\sqrt{5}$$ then $$(a-b\sqrt 5)^n = c - d \sqrt{5}$$ where $a,b,c,d \in \mathbb Z$. This means that:
$$
F_n =\frac{2d \sqrt 5}{2^n \sqrt 5} = \frac{d }{2^{n-1}}.
$$
And it is relatively easy to calculate $d$ using again *binary exponentiation*.

## Fast squaring
Lastly we will use fast squaring which is based on:
$$  \begin{pmatrix}
F_{2n+1} & F_{2n} \\
F_{2n} & F_{2n-1}
\end{pmatrix}
=\begin{pmatrix}
1 & 1 \\
1 & 0
\end{pmatrix}^{2n}
=\begin{pmatrix}
1 & 1 \\
1 & 0
\end{pmatrix}^n\begin{pmatrix}
1 & 1 \\
1 & 0
\end{pmatrix}^n
=\begin{pmatrix}
F_{n+1} & F_{n} \\
F_{n} & F_{n-1}
\end{pmatrix}^2$$
$$
=\begin{pmatrix}
F_{n+1}^2+F_n^2& F_n(F_{n+1}+F_{n-1}) \\
F_n(F_{n+1}+F_{n-1}) & F_n^2 + F_{n-1}^2
\end{pmatrix}
=\begin{pmatrix}
F_{n+1}^2+F_n^2& F_n(2F_{n+1}-F_n) \\
F_n(2F_{n+1}-F_n) & F_n^2 + F_{n-1}^2
\end{pmatrix}.
$$



