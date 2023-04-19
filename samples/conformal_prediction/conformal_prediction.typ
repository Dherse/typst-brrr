#import "slides.typ": *
#show: slide_deck
#set list(indent: 1em)
#set enum(indent: 1em)
#set page(footer: [
  #locate(loc => {
    let k = counter(page).at(loc).first() - 1;
    let n = counter(page).final(loc).first() - 2;
    [
      #set text(size: 12pt, fill: luma(80))
      #set align(right + bottom)
      #if k > 0 [
        #k / #n
      ]
    ]
  })
  #v(0.5em)
])

#let env(head, content) = [
    #head \
    #box(width: 100%, inset: (left: 0.5em))[ #content ]
]
#let thm(kind) = (name, proposition) => [
    #env[
        #smallcaps[#kind] (_#{name}_):
    ][#proposition]
]
#let proof(content) = [
    #env[
        _Proof sketch:_
    ][
        #content
        #v(-1.5em)
        #align(right)[$qed$]
    ]
]
#let definition = thm[Definition]
#let lemma = thm[Lemma]
#let theorem = thm[Theorem]
#let comment(comment) = [
    #set align(center)
    #set text(size: 28pt)
    *#comment*
]

#let proscons(pros, tail_pros, cons, tail_cons) = [
  #columns(2)[
    ==== Pros

    #steplist(start: 0)[#pros]
    #tail_pros

    #colbreak()

    ==== Cons

    #steplist(start: 0)[#cons]
    #tail_cons
  ]
]

#let TODO(message) = [
    #box(fill: red.lighten(80%), inset: 0.5em, radius: 3pt)[
        #env[*#smallcaps[To Do:]*][#message]
    ]
]

#let Var = $op("Var")$
#let ind(x) = $bb(1){#x}$
#let quantile(p, x) = $hat(q)_(#p)(#x)$
#let tquantile(p, x) = $Q_(#p)(#x)$
#let implies = $arrow.r.double.long$
#let impliedby = $arrow.l.double.long$
#let iff = $arrow.l.r.double.long$
#let dist = $upright(d)$

#slide[
    #set align(horizon)
    #set align(center)
    #set text(size: 54pt)
    *A Brief Overview of* \
    *Conformal Prediction*

    #set text(size: 30pt)
    Daniel Csillag \
    #set text(size: 20pt)
    EMAp FGV \
    (ex-IMPA)
]

#slide[
    = The situation

    #steplist(start: 0)[
        - We have models which are fairly good at giving point predictions
        - ... but they are too complex, so we can't reason about them
        - They are _so_ complex that we need to treat them mostly as black-boxes
        - But we _do_ need to have some guarantees on our models
            - E.g., Are our models accurate? Are our models fair? Are we using our models correctly?
        // - What then?
    ]
]

#slide[
    = Conformal Prediction

    A promising approach for getting *probabilistic guarantees* about *arbitrary models*, with fairly *minimal assumptions*.

    //#step[
    //  Not very mainstream yet, but not fringe
    //]

    #step[
      Overview of this presentation:

      1. Some preliminaries
      #v(1fr)
      2. Basic Conformal Prediction procedures // Split Conformal Prediction & Jacknife
      #v(1fr)
      3. Choices of non-conformity scores
      #v(1fr)
      4. Understanding our guarantees
      #v(1fr)
      // 5. Generalizations of Conformal Prediction
      // 5. Some practical examples
      5. _Bonus:_ Dealing with non-exchangeability
      #v(1fr)
    ]
]

#slide[
    = Before we continue

    #set align(center)
    #set align(horizon)
    #v(-2em)
    === Disclaimer
    I'm a CS guy, not a math guy!
]

// #slide[
//   = A quick refresher
// 
//   #definition[Empirical quantile][
//     $ quantile(phi.alt, Z_(1:n)) := inf {t in RR : 1/n sum_(i=1)^n ind(Z_i <= t) >= phi.alt}. $
//   ]
// 
//   #TODO[Image illustrating quantile]
// ]

#slide[
  = Exchangeability

  #definition[Exchangeability][
    Random variables $Z_1, ..., Z_n$ are *exchangeable* if, for any permutation $pi$,

    $ (Z_1, ..., Z_n) limits(=)^d (Z_(pi(1)), ..., Z_(pi(n))). $

    #step[
      I.e., that the cumulative joint distribution of $(Z_1, ..., Z_n)$ is symmetric.
    ]
  ]

  #show: step
  Note that:

  $
    step("iid" &implies "exchangeable")
    \
    step("exchangeable" &implies "identically distributed")
    \
    // step("exchangeable" &implies "identically distributed")
  $

  //#steplist[
  //  - $ "iid" implies "exchangeable" $
  //  - $ "exchangeable" implies "identically distributed" $
  //  // - $ "exchangeable" implies "uncorrelation" $
  //]
]

#slide[
  = Exchangeability

  #lemma[Exchangeability preserving transformations][
    Let $Z_1, ..., Z_n in cal(Z)$ be exchangeable random variables, and $f : cal(Z) -> cal(W)$.
    $f(Z_1), ..., f(Z_n)$ are also exchangeable random variables.
  ]

  #v(1fr)

  #step[
    #lemma[Exchangeability under data splitting][
      Let $Z_1, ..., Z_n, ..., Z_(n+m) in cal(Z)$ be a vector of exchangeable random variables.
      For any function $hat(M)$ that depends arbitrarily on $Z_1, ..., Z_n$,
      $hat(M)(Z_(n+1)), ..., hat(M)(Z_(n+m))$ are also exchangeable random variables.
    ]
  ]

  #v(1fr)

  #step[
    #set align(center)
    (Think train&validation sets.)
  ]
  #v(1fr)
]

#slide[
  = Exchangeability

  #lemma[Exchangeability preserving transformations][
    Let $Z = (Z_1, ..., Z_n) in cal(Z)^n$ be a vector of exchangeable random variables and $G : cal(Z)^n -> cal(W)^m$ be an arbitrary transformation.
    Moreover, suppose that, for each permutation $pi_1 : [m] -> [m]$, there exists a permutation $pi_2 : [n] -> [n]$ such that

    $ pi_1 G(Z) limits(=)^d G(pi_2 Z). $

    Then, the transformation $G$ preserves exchangeability.
  ]
]

#slide[
  = A very useful lemma

  #lemma[Quantile lemma, exchangeable case][
    Let $Z_1, ..., Z_(n+1)$ be exchangeable random variables. Then, given $phi.alt in (0, 1)$,

    $ PP[Z_(n+1) <= quantile(phi.alt, Z_(1:n) union {infinity})] >= phi.alt. $
  ]

  #show: step

  #proof[
    $
      Z_(n+1) &<= quantile(phi.alt, Z_(1:n) union {infinity})
      iff Z_(n+1) <= quantile(phi.alt, Z_(1:n+1))
    $

    #show: step

    $
        therefore
        PP[Z_(n+1) <= quantile(phi.alt, Z_(1:n) union {infinity})]
            &= PP[Z_(n+1) <= quantile(phi.alt, Z_(1:n+1))]
            step(&>= ceil(phi.alt (n+1)) / (n+1) >= phi.alt. )
    $
  ]
]

#slide[
  = How is this useful?

  // We have $n_"train"$ samples $(X^"train"_i, y^"train"_i)$ and $n_"cal"$ samples $(X^"cal"_i, y^"cal"_i)$.

  #steplist(start: 0)[
    1. Train a model $hat(M) : cal(X) -> RR$ on your training data
    2. Evaluate the _nonconformity scores_ $s_i := |hat(M)(X^"cal"_i) - y^"cal"_i|$
    3. $hat(t)_phi.alt := quantile(phi.alt, s_(1:n_"cal") union {infinity})$
    4. $hat(C)_phi.alt(X) := {y in RR : |hat(M)(X) - y| <= hat(t)_phi.alt}$
  ]

  #step[
    *Result:*
    #v(-2em)

    $ PP[y^"test" in hat(C)_phi.alt(X^"test")] >= phi.alt. $
  ]

  #v(-0.5em)

  #show: step
  $
    PP[y^"test" in hat(C)_phi.alt(X^"test")] &= PP[s(X^"test", y^"test") <= hat(t)_phi.alt]
  \ &step(= PP[s(X^"test", y^"test") <= quantile(phi.alt, s_(1:n_"cal") union {infinity})])
    step(>= phi.alt. )
  $
]

#slide[
  = Split Conformal Prediction

  #steplist(start: 0)[
    1. Train a model $hat(M) : cal(X) -> cal(Y)$ on your training data
    2. Evaluate the _nonconformity scores_ $s_i := s(X^"cal"_i, y^"cal"_i)$ \
       (We used $s(X, y) := |hat(M)(X) - y|$.)
    3. $hat(t)_phi.alt := quantile(phi.alt, s_(1:n_"cal") union {infinity})$
    4. $hat(C)_phi.alt(X) := {y in cal(Y) | s(X, y) <= hat(t)_phi.alt}$
  ]

  #step[
    #v(0.5fr)
    *Result:*
    #v(-2em)
  
    $ PP[y^"test" in hat(C)_phi.alt(X^"test")] >= phi.alt. $
  ]

  #v(2fr)
]

#slide[
  = Remarks on SCP

  #proscons(
    [
      - No assumptions on the underlying model
      - No assumptions on the distribution of the data
      - Easy to implement
      - Very cheap
    ],
    [],
    [
      - Interval sizes depend directly on the choice of nonconformity score function
      - We need to split the data
      - We need exchangeability
    ],
    v(2fr),
  )
]

#slide[
  = Full conformal prediction

  Whenever we are making a prediction for a new $X^"test"$,

  #steplist(start: 0)[
    1. For every $y in cal(Y)$, train models $hat(M)_y$ on $(X^"train"_(1:n), y^"train"_(1:n)) union {(X^"test", y)}$
    #box[Let $s_i(y) := |hat(M)_y(X^"train"_i) - y^"train"_i|$ and $t_(1-alpha)(y) := quantile(1-alpha, s_(1:n)(y) union {infinity})$;]
    2. $hat(C)_(1-alpha)(X) := {y in cal(Y) : s(X^"test", y^"test") <= t_(1-alpha)}$
  ]

  #step[
    #v(0.5fr)
    *Result:*
    #v(-2em)
  
    $ PP[y^"test" in hat(C)_(1-alpha)(X^"test")] >= 1 - alpha. $
  ]

  #v(2fr)
]

#slide[
  = Remarks on FCP

  #proscons(
    [
      - No assumptions on the distribution of the data
      - No need to split the data
      // - #strike[Statistical efficiency]
    ],
    v(4fr),
    [
      - The underlying model needs to be symmetric
      - Absurdly expensive
      - Interval sizes depend not only on the nonconformity score function, _but also on the model_
      - We need exchangeability
    ],
    []
  )
]

#slide[
  = The Jackknife+

  #steplist(start: 0)[
    1. Train leave-one-out models $hat(M)_(-i)$ for each training data point $i$
    2. Evaluate $s_i := |hat(M)_(-i)(X^"train"_i) - y^"train"_i|$
    #box[Let $R^-_i(X) := hat(M)_(-i)(X) - s_i$, $R^+_i(X) := hat(M)_(-i)(X) + s_i$;]
    3. $hat(C)_(1-alpha)(X) := [quantile(1-alpha, R^-_(1:n_"cal")(X) union {infinity}), quantile(1-alpha, R^+_(1:n_"cal")(X) union {infinity})]$
  ]

  #step[
    #v(0.5fr)
    *Result:*
    #v(-2em)
  
    $ PP[y^"test" in hat(C)_(1-alpha)(X^"test")] >= 1 - 2alpha. $
  ]

  #step[
    #set align(center)
    But we typically get coverage of about $1 - alpha$.
  ]

  #v(2fr)
]

#slide[
  = Remarks on Jackknife+

  #proscons(
    [
      - No assumptions on the underlying model
      - No assumptions on the distribution of the data
      - No need to split the data
    ],
    v(2em),
    [
      - Lower worst-case coverage
      - Mildly expensive
      - Interval sizes depend directly on the choice of nonconformity score function
      - We need exchangeability
    ],
    [] // v(2fr),
  )
]

#slide[
  = Cross-conformal prediction

  #steplist(start: 0)[
    1. Split the data into $K$ disjoint subsets $S_k$
    2. Train models $hat(M)_(-k)$ on each split $k$
    3. Evaluate $s_i := |hat(M)_(-k)(X^"train"_i) - y^"train"_i|$ for $i in S_k$
    #box[Let $R^-_i(X) := hat(M)_(-k)(X) - s_i$, $R^+_i(X) := hat(M)_(-k)(X) + s_i$ for $i in S_k$;]
    4. $hat(C)_(1-alpha)(X) := [quantile(1-alpha, R^-_(1:n_"cal")(X) union {infinity}), quantile(1-alpha, R^+_(1:n_"cal")(X) union {infinity})]$
  ]

  #step[
    #v(0.5fr)
    *Result:*
    #v(-2em)
  
    $ PP[y^"test" in hat(C)_(1-alpha)(X^"test")] >= 1 - 2alpha. $
  ]

  #step[
    #set align(center)
    But, again, we typically get coverage of about $1 - alpha$.
  ]

  #v(1fr)
]

#slide[
  = Remarks on CV+

  #proscons(
    [
      - No assumptions on the underlying model
      - No assumptions on the distribution of the data
      - No need to split the data
    ],
    v(2em),
    [
      - Lower worst-case coverage
      - Slightly expensive
      - Interval sizes depend directly on the choice of nonconformity score function
      - We need exchangeability
    ],
    [] // v(2fr),
  )
]

#slide[
  = Scores for regression

  We've already considered $s(X, y) = |hat(M)(X) - y|$.

  #step[However, note:]

  #steplist(start: 0)[
    - $s$ gives predictive intervals that are symmetric around $hat(M)(X)$;
    - ... and they are of constant size!
  ]

  #v(2fr)
]

#slide[
  = Overcoming symmetry

  #step[
    *Idea:* use separate quantiles for upper and lower bounds.
  ]

  //#step[
  //  #columns(2)[
  //    $ s^-(X, y) := min {hat(M)(X) - y, 0} $
  //    #colbreak()
  //    $ s^+(X, y) := max {hat(M)(X) - y, 0} $
  //  ]
  //]

  #columns(2)[
    #step[
      $ hat(t^+) := quantile(1 - alpha\/2, s_(1:n_"cal")) $
    ]

    #step[
      $ PP[s(X^"test", y^"test") <= hat(t^+)] >= 1-alpha/2 $
    ]

    #colbreak()

    #step[
      $ -hat(t^-) := quantile(1-alpha\/2, -s_(1:n_"cal")) $
    ]

    #step[
      $ PP[-s(X^"test", y^"test") <= -hat(t^-)] >= 1 - alpha/2 $
    ]

    #step[
      $ PP[hat(t^-) <= s(X^"test", y^"test")] >= 1 - alpha/2 $
    ]
  ]

  #v(-1em)

  #step[
    $ hat(C)_(1-alpha)(X) := {y in cal(Y) : hat(t^-) <= s(X, y) <= hat(t^+)} $
  ]

  #step[
    $ PP[y^"test" in hat(C)_(1-alpha)(X^"test")] = PP[hat(t^-) <= s(X^"test", y^"test") <= hat(t^+)] >= 1 - alpha. $
  ]
]

#slide[
  = Adaptive intervals

  #step[
    Suppose we have some intuitive measure of uncertainty, $u : cal(X) -> RR^+$.
  ]

  #step[
    We can use
    //#v(-2em)
    $ s(X, y) := |hat(M)(X) - y|/u(X). $
  ]

  //#step[
  //  #TODO[image demonstrating adaptive intervals]
  //]
]

#slide[
  = Quantile regression

  We can use quantile regression to attempt to learn:

  - $hat(M)^-$ modelling $tquantile(alpha\/2, Y | X)$; and
  - $hat(M)^+$ modelling $tquantile(1 - alpha\/2, Y | X)$.

  #step[
    However, $PP[hat(M)^-(X^"test") <= y^"test" <= hat(M)^+(X^"test")] >= 1 - alpha$ typically won't hold out of the box...
    So let's calibrate it with conformal prediction!
  ]

  #step[
    $ s(X, y) := max {hat(M)^-(X) - y, y - hat(M)^+(X)}. $
  ]
]

#slide[
  = Scores for classification

  #step[
    How about

    $ s(X, y) := ind(hat(M)(X) = y)? $
  ]

  #step[
    Just like $s(X, y) := |hat(M)(X) - y|$, this is not great:
  ]

  #steplist(start: 0)[
    - Predictive sets can either contain a single $y$ or all $y in cal(Y)$;
    - ... and even then, they are all of constant size!
  ]

  #step[
    But, just like with regression, we can do _much_ better!
  ]
]

#slide[
  = Logits as scores

  Suppose we have a classification model outputting logits, $hat(M) : cal(X) -> [0, 1]^K$.

  #step[
    We can use $s(X, y) := [hat(M)(X)]_y$.
  ]
  #step[
    #h(0.25em) But we can do better!
  ]

  #step[
    If $hat(M)(X)$ were a perfect model of $Y | X$, then we'd take $y in cal(Y)$ greedily just until $sum_y [hat(M)(X)]_y >= 1-alpha$.
  ]

  #steplist(start: 0)[
    1. Sort the logits in decreasing order
    2. $β <- 0$
    3. For each [sorted] logit $p$ for class $i$: \
    4. #h(1em) $β <- β + p$, add class $i$ to set
    5. #h(1em) If $β >= 1 - alpha$, then break
  ]

  //#step[
  //  #v(-0.9em)
  //  #set align(center)
  //  #image("./cumulative-logits-trimmed.png", height: 8em, fit: "contain")
  //]
]

#slide[
  = Conformalized Bayes

  #step[
    Suppose we have a Bayesian model $hat(M)(y | X)$.
  ]

  #step[
    The Bayesian apporach would be to use predictive sets

    $ S(X) = {y in cal(Y) : hat(M)(y | X) > t}, $

    With $t$ such that $integral_(y in S(X)) hat(M)(y | X) dif y = 1 - alpha$.
  ]

  #step[
    For Conformal Prediction, we can just use $s(X, y) := -hat(M)(y | X)$.
  ]

  #step[
    With some additional (not weak) assumptions, this set has the smallest average size for $1-alpha$ coverage, over data _and_ parameters.
  ]
]

#slide[
  = Batch setting

  Our guarantee:
  #v(-2em)
  $ PP[y^"test" in hat(C)_(1-alpha)(X^"test")] >= 1 - alpha $

  #step[What about more than one prediction?]
  #step[
    Given exchangeable $X^"test"_1, y^"test"_1, dots$:

    $ 1/T sum_(t=1)^T ind(y^"test"_t in hat(C)_(1-alpha)(X^"test"_t)) limits(-->)^(T -> infinity) 1 - alpha. $
  ]

  //#step[
  //  #set align(right)
  //  $implies$ a good way of checking empirically that coverage holds
  //]
  #step[
    We can also get a concentration measure version if we assume iid data:

    $ PP[1/T sum_(t=1)^T ind(y^"test"_t in hat(C)_(1-alpha)(X^"test"_t)) >= 1-alpha] >= 1-delta. $
  ]

  // show how the standard theorem already solves this with exchangeability
  // show concentration measure version
]

#slide[
  = Batch setting

  Suppose, for $R$ different test sets with $T$ points each, we evaluate

  $
    C_j := 1/T sum_(t=1)^T ind(y^("test",j) in hat(C)_(1-alpha)(X^("test",j))),
    quad quad quad
    overline(C) = 1/R sum_j C_j;
  $

  #step[
    We will have that

    $
      // EE[overline(C)] &approx 1 - alpha // &= 1 - l/(n+1)
      // \
      sqrt(Var[overline(C)]) &= cal(O)(1/sqrt(R min {n_"cal", T})) step(limits(arrows.rr)^(R -> infinity)_(n_"cal",T -> infinity) 0)
    $
  ]
]

#slide[
  = Marginal coverage

  #columns(2)[
    $ PP[y in hat(C)_(1-alpha)(X)] >= 1 - alpha $

    #colbreak()

    #step[
      #set align(center)
      *This is marginal coverage!*
    ]
  ]

  #step[
    #columns(2)[
      $ PP[y in hat(C)_(1-alpha)(X) | X] >= 1 - alpha $

      #colbreak()

      #set align(center)
      This is conditional coverage.
    ]
  ]

  #step[
    I.e.,

    #v(-2em)
    $ PP[y in hat(C)_(1-alpha)(X) | X = x] >= 1 - alpha quad quad forall x in cal(X) $
  ]

  // #TODO[an image illustrating how marginal coverage fails (probably just a plot of a normal distribution)]

  // single-sample setting
  // batch setting
  // the issues with marginal coverage
]

#slide[
  = Conditional coverage

  #step[
    Impossible in general :/ // #emoji.face.sad
    // @article{Barber2019TheLO,
    //  title={The limits of distribution-free conditional predictive inference},
    //  author={Rina Foygel Barber and Emmanuel J. Cand{\`e}s and Aaditya Ramdas and Ryan J. Tibshirani},
    //  journal={Information and Inference: A Journal of the IMA},
    //  year={2019}
    //}
  ]

  #step[
    We can, however, get *class-conditional coverage*:

    $
      &"Given" G_1, ..., G_m "such that" G_1 union.sq dots.c union.sq G_m = cal(X),
    \ &PP[y in hat(C)_(1-alpha)(X) | X in G_i] >= 1 - alpha,
      quad quad forall i = 1, ..., m
    $
  ]

  #steplist(start: 0)[
    1. For each group $G_i$, use standard conformal prediction to get $hat(C)^((i))_(1-alpha)$
    2. Define
       #v(-1em)
       $ hat(C)_(1-alpha)(X) := hat(C)^((i))_(1-alpha)(X), quad X in G_i. $
  ]

  // TODO result

  // impossible in general
  // finite VC dimension case
  // group-conditional CP & multi-valid CP
]

#slide[
  = Non-exchangeability

  Exchangeability is a very strong assumption, actually.

  What if it is unreasonable?

  #step[For example:]

  #steplist(start: 0)[
    - Time series
    - Change points
    - Autoregressive models
    - Interactive systems
  ]
]

#slide[
  = No clear answer yet

  This is still an open question, with plenty of literature proposing solutions.

  #step[
    However, it's clear that to be able to deal with arbitrary data we need to operate in an on-line manner.
  ]

  #step[
    #set align(center)
    #v(-0.9em)
    #image("./enbpi.png", height: 10em, fit: "contain")
  ]
]

#slide[
  = Theoretical bounds

  ==== Conformal prediction beyond exchangeability

  #lemma[Quantile lemma, nonexchangeable&symmetric case][
      Let $Z = (Z_1, ..., Z_(n+1))$ be a vector of random variables. Given $phi.alt in (0, 1)$,

      $ PP[Z_(n+1) <= quantile(phi.alt, Z_(1:n) union {infinity})] >= phi.alt - (sum_(i=1)^n dist_"TV"(Z, Z^((i))))/(n+1), $

      // Where $Z^((i)) := (underbrace(#[$Z_1, ..., Z_(i-1)$], Z_(1:i-1)), Z_(n+1), underbrace(#[$Z_(i+1), ..., Z_n$], Z_(i+1:n)), Z_i)$.
      Where $Z^((i)) := (Z_1, ..., Z_(i-1), Z_(n+1), Z_(i+1), ..., Z_n, Z_i)$.
  ]

  #step[ *Note:* if $Z_1, ..., Z_(n+1)$ are exchangeable, then $(sum_(i=1)^n dist_"TV"(Z, Z^(i)))/(n+1) = 0$. ]
]

#slide[
  = Theoretical bounds

  ==== Conformal prediction beyond exchangeability

  #lemma[Quantile lemma, nonexchangeable&weighted case][
      Let $Z = (Z_1, ..., Z_(n+1))$ be a vector of random variables, and weights $w_1, ..., w_n in RR_(>0)$. Given $phi.alt in (0, 1)$,

      $ PP[Z_(n+1) <= quantile(phi.alt, Z_(1:n) union {infinity}\; w_(1:n) union {1})] >= phi.alt - (sum_(i=1)^n w_i dist_"TV"(Z, Z^((i))))/(sum_(i=1)^n w_i +1). $

      // Where $Z^((i)) := (Z_1, ..., Z_(i-1), Z_(n+1), Z_(i+1), ..., Z_n, Z_i)$.
  ]

  #step[
    *Idea:* use higher weights for more recent points.
  ]
]

//#slide[
//  = Special cases
//
//  - beta-mixing [CITE]
//  #v(1fr)
//  - strongly-mixing [CITE]
//  #v(1fr)
//  - weakly-mixing [CITE]
//  #v(1fr)
//]

//#slide[
//  = Adaptive Conformal Intervals
//
//  ==== Gibbs-Candès
//
//  *Idea:* if we can't evaluate the coverage gap, let's continuously estimate it
//
//  #step[
//    Let's start with $alpha_0 <- alpha$.
//  ]
//
//  #step[
//    Every time we receive a new $(X^"test", y^"test")$, we update
//
//    $ alpha_(t+1) <- alpha_t + gamma (alpha - ind(y^"test" in.not hat(C)_(alpha_t)(X^"test"))). $
//  ]
//
//  #step[
//    #v(0.5fr)
//    *Result:*
//    #v(-3em)
//
//    $ 1/T sum_(t=1)^T ind(y^"test"_t in hat(C)_(1-alpha)(X^"test"_t)) limits(-->)^(T -> infinity)_(a.s.) 1 - alpha $
//  ]
//]
//
//#slide[
//  = Adaptive Conformal Intervals
//
//  Note that
//
//  $
//    alpha_T = alpha_0 + gamma sum_(t=1)^T (alpha - ind(y^"test"_t in.not hat(C)_(1-alpha)(X^"test"_t)))
//    step(<= 1 + gamma)
//  $
//
//  #step[
//    $
//      1/T sum_(t=1)^T (alpha - ind(y^"test"_t in.not hat(C)_(1-alpha)(X^"test"_t)))
//      <= (1 + gamma - alpha_0)/(T gamma)
//    $
//  ]
//
//  #show: step
//  $
//    alpha - 1/T sum_(t=1)^T ind(y^"test"_t in.not hat(C)_(1-alpha)(X^"test"_t))
//    <= (1 + gamma - alpha_0)/(T gamma)
//    step(limits(-->)^(T -> infinity) 0)
//  $
//]
//
//#slide[
//  = Adaptive Conformal Intervals
//
//  #TODO[
//    Tricky choice of $gamma$
//    Usually produces many infinite intervals
//  ]
//]

//#slide[
//  = AggACI
//
//  #TODO[
//    - Josse on Gibbs-Candes
//    - Frustratingly rare in benchmarks due to complexity :\(
//  ]
//]
//
//#slide[
//  = EnbPI
//
//  #TODO[explain]
//]
//
//#slide[
//  = MVP
//
//  #TODO[explain]
//]

#slide[
  = Conclusion

  #steplist(start: 0)[
    - Conformal Prediction is a promising approach for distribution-free uncertainty quantification on arbitrary models
    - Coverage is guaranteed, as long as assumptions hold
    - Efficiency depends on the nonconformity scores and actual models
    - Active research on the area, especially regarding non-exchangeability
  ]

  #step[Topics I didn't cover:]

  #steplist(start: 0)[
    - How to compare CP methods
    - Deep dive on non-exchangeability and distribution drift
    - Guarantees of the form $EE[ell(C(X), y)] <= alpha$
    - Other uses of CP techniques
  ]
]

// Some more slides explaining other guarantees with exchangeability
//
//#slide[
//  = Can we go further?
//
//  TODO: cite Bates2021
//
//  Given a loss function $ell : 2^(cal(Y)) times cal(Y) -> cal(Y)$ such that $ell(C, y)$ shrinks as $C$ grows,
//
//  #v(0.5fr)
//  *Result:*
//  #v(-2em)
//
//  $ EE[ell(C_phi.alt(x^"test"), y^"test")] <= alpha. $
//]
//
//#slide[
//  = Can we go further?
//
//  There are a couple of other guarantees explored in the literature.
//
//  #TODO[
//    - Out-of-distribution detection
//    - My unpublished work
//  ]
//]