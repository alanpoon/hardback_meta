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
    pub instructions: Vec<&'static str>,
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
            instructions: vec!["12 cards are given to each players",
                               "Draft phase: Each player must select one card from their own pile and add it to their hand. The rest of cards are passed clockwise",
                               "First Tab shows either draft phase or play a card in your hand. You are given 12 cards, choose one to draft into your hand.",
                               "This prompt panel will tell you what to do in each phase. This is the draft phase",
                               "Each item is a card in your draft pile.",
                               "The icons shown here is what the card gives you. At the end, it also tells you if this card is an action or thug or holding card.",
                               "Select a card by clicking on it.",
                               "You can see the card in full.",
                               "Click Ok to confirm.",
                               "First 2 rounds, there is only draft phase.",
                               "For the rest of 10 rounds, there are Draft phase, play One Hand Card phase, Perform actions phase and Resolve Each Turn phase.",
                               "Great, this is the third round, in play One Hand Card phase, we are going to play one card",
                               "Hint, make sure you fulfill the card's requirement before playing, or else you will have to discard the card",
                               "You can only fuflill the requirement with the cards in play.",
                               "Hint, when you play your hand card, choose something that has no requirement!",
                               "All the players will play their chosen hand card simultaneously, so at the bottom right of the card, there is a number to resolve the sequence.",
                               "Time to click Ok to confirm.",
                               "Click the players tab to see the cards that are in play",
                               "In the players tab, you can also see the number of keys, guns, wrenches etc each player has.",
                               "For Holding cards, there is a number in a bracket that shows the number of development markers on it. Each marker is worth $10,000 at the end of the game.",
                               "One development marker is placed for each icon the holding has, and also for each matching icon the holding has with the other holdings the player has.",
                               "That is all. Good Luck"],
        }
    }
}