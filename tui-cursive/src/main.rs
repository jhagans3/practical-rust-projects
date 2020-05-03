use cursive::Cursive;
use cursive::event::Key;
use cursive::views::{Dialog, TextView};

fn main() {
    let mut siv = Cursive::default();

    let cat_text = "Meow!
 \\
  \\
    /\\_/\\
   ( o o )
   =( I )=";


   siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .button("OK", |s| s.quit())
    );

    // Declaring the app layout
    // siv.add_layer(TextView::new(cat_text));

    // Listen to Key::Esc and quit
    siv.add_global_callback(Key::Esc, |s| s.quit());

    // start event loop
    siv.run();
}
