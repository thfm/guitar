use gitar::{standard_tuning, Guitar, Note, FretDiagram, Size};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Finds the occurences of the given note on a guitar.
    Find {
        note: Note,
        /// The number of frets on the guitar.
        #[structopt(short = "f", long = "frets", default_value = "21")]
        num_frets: usize,
        /// The tuning configuration of the guitar.
        #[structopt(short = "t", long = "tuning")]
        tuning: Option<Vec<Note>>,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Find {
            note,
            num_frets,
            tuning,
        } => {
            // Uses standard tuning if there was no given tuning (or if the given
            // tuning was invalid)
            let tuning = tuning.unwrap_or(standard_tuning());

            let guitar = Guitar::new(num_frets, tuning);

            let locations = guitar.locations(note);
            if locations.len() > 0 {
                println!("{}", FretDiagram::new(locations, Size::Small));
                
            } else {
                println!("No occurences.");
            }
        }
    }

    Ok(())
}
