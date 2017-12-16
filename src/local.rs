#[derive(Debug)]
pub struct Local {
    pub menu: Vec<&'static str>,
    pub pick: &'static str,
    pub players: &'static str,
    pub lobby: &'static str,
    pub chat: &'static str,
    pub log: &'static str,
    pub yes: &'static str,
    pub no: &'static str,
    pub inhand: &'static str,
    pub inplay: &'static str,
    //lobby
    pub newtable: &'static str,
    pub changename: &'static str,
    pub playername: &'static str,
    //tablelist
    pub changeto: &'static str,
    pub playergame: &'static str,
    pub ready: &'static str,
    pub leave: &'static str,
    pub join: &'static str,
    //tutorial
    pub gangland: &'static str,
    pub betherichest: &'static str,
    pub next: &'static str,
    pub previous: &'static str,
    pub skiptutorial: &'static str,
    pub instructions1: Vec<&'static str>,
    pub instructions2: Vec<([f64; 4], Option<[f64; 4]>)>,
    //spell
    pub draw_extra_card:&'static str,
    pub submit:&'static str,
    //overlay
    pub use_ink:&'static str,
    pub use_ink_insufficent:&'static str,
    pub use_remover:&'static str,
    pub use_remover_insufficent:&'static str,
    pub use_timelessclassic:&'static str,
    pub buy:&'static str,
    pub unused_coins:&'static str,
    pub continue_without_buying:&'static str,
    pub trash:&'static str,
    pub trash_other:&'static str,
    pub continue_without_trashing_other:&'static str,
    pub are_you_sure_exit:&'static str,
}

impl Local {
    #[cfg(feature = "english")]
    pub fn new() -> Local {
        Local {
            menu: vec!["Story Mode", "Multiplayer", "Credit"],
            pick: "Pick",
            players: "Players",
            lobby: "Lobby",
            chat: "Chat",
            log: "Log",
            yes: "Yes",
            no: "No",
            inhand: "In Hand",
            inplay: "In Play",
            newtable: "New Table",
            changename: "Change Name",
            changeto: "Change to",
            playername: "Player Name:",
            playergame: "player game",
            ready: "ready",
            leave: "leave",
            join: "join",
            gangland: "GANGLAND",
            betherichest: "Be the Richest at the end of 12 rounds",
            next: "Next",
            previous: "Previous",
            skiptutorial: "Skip Tutorial",
            instructions1: vec!["Work to pen your next masterpiece by earning prestige along the way.",
                "You start with your own personalized deck of cards, using them to acquire better cards and reshuffle your deck throughout the game.",
                "Each Player is dealt with 8 starting purchasing cards, each providing 1 cent, consisting of the letters: A,E,I,L,N,R,S,T and two starting prestige cards, each providing 1 Prestige point, selected at random.",
                                "Each player shuffles their starting deck and draws their first hand of 5 cards",
                                "Place 7 cards in the center, this will be the Offer Row. Each time after a player manages to complete a word, they may purchase a card from this row.",
                                "Next, let's talk about the card basics.",//5
                                "Letter. Use it to spell a word on your turn.",
                                "Genre. The icon in the top-left below the letter. There are four genres.",
                                "Genre. Mystery cards can uncover hidden cards and remove cards from the Offer Row.",
                                "Genre. Horror cards terrify other players by providing the access to the Ink Remover and offering Coin and Prestige flexibility.",
                                "Genre. Adventure cards are focused on quick and steady progress towards victory. They have abilities to obtain immediate prestige or trash the existing card.",//10
                                "Genre. Romance cards can trash other cards and multiply other card Benefits.",
                                "Basic Benefits. You get this benefit if you use this card to form a word.",
                                "Genre Benefits. You only get this benefit if you use more than 1 card from this genre to form a word.",
                                "Cost. The number of coins you need to pay for this card. You may trade 3 ink or ink remover to 1 coin for the card",//14

                               ],
            instructions2:vec![([0.1, 0.05, 0.7, 0.9], None),([0.1, 0.05, 0.7, 0.9], None),([0.1, 0.05, 0.7, 0.9], None),([0.1, 0.05, 0.7, 0.9], None),([0.1, 0.05, 0.7, 0.9], None),([0.1, 0.05, 0.7, 0.9], None),//5
            ([0.1, 0.05, 0.7, 0.9], None),
            ([0.1, 0.05, 0.7, 0.9], Some([0.2,0.2,0.2,0.2])),
            ([0.1, 0.05, 0.7, 0.9], Some([0.2,0.25,0.2,0.2])),
            ([0.1, 0.05, 0.7, 0.9], None),
            ([0.1, 0.05, 0.7, 0.9], None),//10
            ([0.1, 0.05, 0.7, 0.9], None),
            ([0.1, 0.05, 0.7, 0.9], None),
            ([0.1, 0.05, 0.7, 0.9],  Some([0.24,0.3,0.3,0.3])),
            ([0.1, 0.05, 0.7, 0.9],  Some([0.24,0.35,0.3,0.3])),
            ([0.1, 0.05, 0.7, 0.9],  Some([0.2,0.39,0.2,0.2])),//15
            ],
            draw_extra_card:"Draw extra card",
            submit:"submit",
            use_ink:"Use Ink",
            use_ink_insufficent:"You have insufficient ink.",
            use_remover:"Use Ink Remover",
            use_remover_insufficent:"You have insufficient ink remover.",
            use_timelessclassic:"Use Timeless Classic",
            buy:"Buy",
            unused_coins:"Unused coins will be converted into ink. Subsequently, 3 inks have to be used for a value of one coin",
            continue_without_buying:"Continue without buying",
            trash:"Trash Card",
            trash_other:"Pick a Card to trash for one cent",
            continue_without_trashing_other:"Continue without trashing another card",
            are_you_sure_exit:"Are you sure that you want to exit?"
        }
    }
}
