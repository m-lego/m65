use keyberon::action::{k, l, m, Action::*, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<()> = &[
    &[
        
        &[k(Escape)   , k(Kb1)        , k(Kb2), k(Kb3) , k(Kb4), k(Kb5)  , k(Kb6)  , k(Kb7)  , k(Kb8) , k(Kb9)   , k(Kb0)   , k(Minus)   , k(BSpace)]  ,
        &[k(Tab)      , k(Q)          , k(W)  , k(E)   , k(R)  , k(T)    , k(Y)    , k(U)    , k(I)   , k(O)     , k(P)     , k(LBracket), k(RBracket)],
        &[k(NonUsHash), k(A)          , k(S)  , k(D)   , k(F)  , k(G)    , k(H)    , k(J)    , k(K)   , k(L)     , k(SColon), k(Quote)   , k(Enter)]   ,
        &[k(LShift)   , k(NonUsBslash), k(Z)  , k(X)   , k(C)  , k(V)    , k(B)    , k(N)    , k(M)   , k(Comma) , k(Dot)   , k(Up)      , k(Slash) ]  ,
        &[k(LCtrl)    , k(LGui)       , l(1)  , k(LAlt), l(2)  , k(Space), k(Space), k(Space), k(RAlt), k(RShift), k(Left)  , k(Down)    , k(Right)]   ,

    ], &[
    
        &[k(Grave), k(Pause), Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, k(PScreen), k(Delete), k(Equal) ],
        &[Trans   , Trans   , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans     , Trans    , Trans ]   ,
        &[Trans   , Trans   , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans     , Trans    , Trans ]   ,
        &[Trans   , Trans   , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans     , Trans    , Trans ]   ,
        &[Trans   , Trans   , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans     , Trans    , Trans ]   ,
    
    ], &[

        &[k(Escape)  , k(F1), k(F2), k(F3), k(F4), k(F5), k(F6), k(F7), k(F8), k(F9), k(F10), k(F11), k(F12)],
        &[Trans      , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans , Trans , Trans ],
        &[k(CapsLock), Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans , Trans , Trans ],
        &[Trans      , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans , Trans , Trans ],
        &[Trans      , Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans , Trans , Trans ],


    ], &[

        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans ],
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans ],
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans ],
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans ],
        &[Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans, Trans ],

    ],
];
