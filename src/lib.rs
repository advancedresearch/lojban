extern crate piston_meta;

#[cfg(test)]
mod tests {
    use piston_meta::*;

    fn pass(text: &str) {
        let rules = include_str!("../assets/syntax.txt");
        // Parse rules with meta language and convert to rules for parsing text.
        let rules = match syntax_errstr(rules) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(rules) => rules
        };
        let mut data = vec![];
        match parse_errstr(&rules, text, &mut data) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(()) => {}
        };
        json::print(&data);
    }

    fn fail(text: &str) {
        let rules = include_str!("../assets/syntax.txt");
        // Parse rules with meta language and convert to rules for parsing text.
        let rules = match syntax_errstr(rules) {
            Err(err) => {
                panic!("{}", err);
            }
            Ok(rules) => rules
        };
        let mut data = vec![];
        match parse_errstr(&rules, text, &mut data) {
            Err(_) => {}
            Ok(()) => {
                panic!("Invalid `{}`", text);
            }
        };
        // json::print(&data);
    }

    #[test]
    fn test_pass() {
        // I give this to you.
        pass("mi dunda ti do");
        // I sell this to you.
        pass("mi vecnu ti do");
        // The seller is friend of the giver.
        pass("le vecnu ku pendo le dunda ku");
        // The giver talks to the friend.
        pass("le dunda ku tavla le pendo ku");
        // The friend talks to me.
        pass("le pendo ku tavla mi");
        // The talker gives this to the seller.
        pass("le tavla ku dunda ti le vecnu ku");
        // The giver sells that to the seller.
        pass("le dunda ku vecnu ta le vecnu ku");
        // I give this.
        pass("mi dunda zo'e");
        // I give this to John.
        pass("mi dunda ti la John");
        // I sell this to Jhon for 10.
        pass("mi vecnu ti la John 10");
        // I talk to you about something in some language.
        pass("mi tavla do zo'e zo'e");
        // You talk to me about that thing in some language.
        pass("do tavla mi ta zo'e");
        // I talk to someone about that thing yonder in this language.
        pass("mi tavla zo'e tu ti");
        // I sell this thing to that buyer.
        pass("mi vecnu ti ta zo'e");
        // I this thing do sell to that buyer.
        pass("mi ti vecnu ta");
        // I this thing to that buyer do sell.
        pass("mi ti ta vecnu");
        // That is beautiful.
        pass("ta melbi");
        // Beautiful!
        pass("melbi");
        // I talk to you about this.
        pass("mi tavla do ti");
        // You are talked to me about this.
        pass("do se tavla mi ti");
        // This is talked to you by me.
        pass("ti te tavla do mi");
        // You are talked about this by me.
        pass("do te se tavla ti mi");
        // I talk to you and you talk to me.
        pass("mi tavla do .i do tavla mi");
        // I talk to you. You talk to me.
        pass("mi tavla do. do tavla mi");
        // I talk to you.. you talk to me.
        pass("mi tavla do ni'o do tavla mi");
        // I quickly-go to Mary.
        pass("mi sutra klama la meris");
        // Tom beautifully-talks to Mary.
        pass("la tom melbi tavla la meris");
        // Mary is beautifully-talked-to by Tom.
        pass("la meris melbi se tavla la tom");
        // Mary is audiencely-beautiful to Tom.
        pass("la meris se melbi tavla la tom");
        // I talk about the summer.
        pass("mi tavla zo'e le glacitsi ku");
        // I talk to you about the talker.
        pass("mi tavla do le tavla");
        // I give you the fast talker.
        pass("mi dunda le sutra tavla do");
        // The fast one is talking.
        pass("le sutra cu tavla");
        // The fast talked-to one is talking.
        pass("le sutra se tavla cu tavla");
        // The fast one is talked to.
        pass("le sutra cu se tavla");
        // I am talking to the seller about the blue-green thing.
        pass("mi tavla la vecnu le blari'o");
        // The dog is beautiful.
        pass("le gerku cu melbi");
        // This pleases me.
        pass("ti pluka mi");
        // The last sentence pleases me.
        pass("di'u pluka mi");
        // The meaning of the last sentence pleases me.
        pass("la'e di'u pluka mi");
        pass("la'edi'u pluka mi");
        // My dog is fast.
        pass("le mi gerku cu sutra");
        // Who is talking to you about me?
        pass("ma tavla do mi");
        // You are doing what?
        pass("do mo");
        // Are you talking to me?
        pass("xu do tavla mi");
        // Yes (restating the asked question).
        pass("go'i");
        // Are you healthy?
        pass("xu do kanro");
        // I am healthy.
        pass("mi kanro");
        // The talker is healthy.
        pass("le tavla cu kanro");
        // Yes, the talker is healthy.
        pass("le tavla cu go'i");
        // Not true.
        pass("na go'i");
        // Agreed. I should go.
        pass(".ie mi klama");
        // John went to the store.
        pass("la djan pu klama la zarci");
        // John goes to the store.
        pass("la djan ca klama la zarci");
        // You are selling something far away.
        pass("do vu vecnu zo'e");
        // The yonder runner talks.
        pass("le pu bajra cu tavla");
        // This runner talks.
        pass("le vi bajra cu tavla");
        // This talker will go.
        pass("le vi tavla cu ba klama");
        // I am happy because it is the beautiful thing I am going to.
        pass("mi klama le melbi .ui");
        // I go and you stay.
        pass("mi klama .i ji'a do stali");
        // I go, however you stay.
        pass("mi klama .i ku'i do stali");
        // The one that is beautiful is talking.
        pass("lo melbi ku tavla");
    }

    #[test]
    fn test_fail() {
        // `cu` requires semti before selbri.
        fail("cu melbi");
    }

    #[test]
    fn test_print() {
        // Your dog that is me is fast.
        pass("mi dunda ti do");
        // panic!("");
    }
}
