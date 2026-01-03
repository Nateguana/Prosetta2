#![cfg(not(feature = "wasm"))]

use std::{env::args, hint::black_box, mem};

use itertools::Itertools;
// use parking_lot::lock_api::Mutex;
use parser::{
    tree_writer::{term_writer::TermWriter, TreeAllWriter, TreeWriter},
    ParserSource,
};

mod parser;
use crate::parser::Parser;

// // #[path = "testing/testing.rs"]
// mod testing;
//mod playground;

// mod parser_runner;

// mod commands;
// mod docs_lib;
// mod writers;

// use docs_lib::{gen_output, gen_test};
// use parser::ParserSource;
// use parser_runner::{run_parser, RunnerFlags};

// fn generate_tests() {
//     println!("Generating JS output...");
//     let paths = fs::read_dir("../Frontend/docs").unwrap();
//     let mut path_strs: Vec<String> = Vec::new();

//     for p in paths {
//         match p {
//             Ok(v) => {
//                 let path = v.path();
//                 // We're only using ASCII so I think this is fine
//                 let path_str = str::replace(&path.display().to_string(), "\\", "/");

//                 if path.is_file() && path_str.ends_with(".md") {
//                     path_strs.push(path_str.clone());
//                     gen_output(&path_str);
//                 }
//             }
//             Err(e) => println!("{}", e),
//         }
//     }
//     path_strs.sort();
//     let test_doc_path = "src/testing/test_docs.rs";
//     let contents = fs::read_to_string(test_doc_path).expect("File not found");
//     let start_str = "// START OF GENERATED TESTS";
//     let end_str = "// END OF GENERATED TESTS";
//     let start_idx = contents.find(start_str).unwrap() + start_str.len();
//     let end_idx = contents.find(end_str).unwrap();
//     let test_str = path_strs
//         .iter()
//         .map(gen_test)
//         .collect::<Vec<String>>()
//         .join("\n");
//     _ = fs::write(
//         test_doc_path,
//         format!(
//             "{}{}{}",
//             &contents[..start_idx],
//             test_str,
//             &contents[end_idx..]
//         ),
//     );
// }

fn main() {
    // if cfg!(feature = "gen-doc-output") {
    //     generate_tests();
    //     return;
    // }
    println!("size of parser: {}", mem::size_of::<Parser>());

    // let mut args: Vec<String> = std::env::args().skip(1).collect();

    // for e in &mut args {
    //     e.make_ascii_lowercase();
    // }

    //args.sort();

    // let t = Mutex::new(1 as u64);

    // let k = mem::size_of::<Mutex<u64>>();

    let debug_program = false;
    // !args().skip(1).any(|e| e.contains("r"));

    let source = ParserSource::from_stdin();
    let parser = Parser::new(source);

    if black_box(debug_program) {
        // debug(parser);
    } else {
        run(parser);
    }

    // let ParserAction::Finished { data } = last_step.unwrap().action else {
    //     unreachable!()
    // };

    // println!("{:?}", data);

    // let parser_flags = ParserFlags {
    //     title: !cfg!(feature = "no-title"), //args.binary_search(&"not".to_string()).is_ok(),
    // };

    // let vis_flags: RunnerFlags = RunnerFlags {
    //     assert_steps: true,
    //     input: true,
    //     whole_program: true,
    //     linted: true,
    //     line_rendered: true,
    //     word_trigger: true,
    // };

    // run_parser(
    //     parser_flags,
    //     vis_flags,
    //     ParserSource::from_stdin(), //ParserSource::from_string(MILO_POEM_2[0].to_vec())
    // );

    // let _ = io::stdin().read(&mut [0u8]).unwrap();
}

// fn debug(parser: Parser) {
//     let mut debugger = parser.debug();

//     // let mut last_step = None;
//     while let Some(step) = debugger.next() {
//         println!("{:?}", step);
//         println!("{}", TreeAllWriter::write_all_lisp(&debugger.tree()));

//         let mut writer = TermWriter::new();
//         while writer.step(&debugger.tree()) {
//             let str = writer.next(&debugger.get_source());
//             println!("{}", str);
//         }
//         // for paragraph in debugger.get_source().get_iter() {
//         //     println!("{:?}", str::from_utf8(paragraph).unwrap());
//         // }
//         println!("{}", TreeAllWriter::write_all_javascript(&debugger.tree()));
//         println!("--------------------------------");
//     }
// }

fn run(parser: Parser) {
    let parser_data = parser.run();

    println!("--------------------------------");
    // println!("{:?}", parser_data.source);
    let mut writer = TermWriter::new();

    let paragraph_starts  = parser_data.tree.get(1).get_children();
    for starts in paragraph_starts {
        writer.step(&parser_data.tree, starts);
        let str = writer.next(&parser_data.source);
        println!("{}", str);
    }
    println!("{}", TreeAllWriter::write_all_javascript(&parser_data.tree));
    println!("{}", TreeAllWriter::write_all_lisp(&parser_data.tree));
}
// #[allow(dead_code)]
// static MILO_POEM: [&[u8]; 2] = [
//     b"
// The wizards utter 'paint iambically.'
// The peasants hadn't choice but to obey.
// \
// The wizards' cruel entertainments chant
// and utter utter nonsense, void of weight.
// \
// The wizards' cursed victims utter trash;
// the mages stand offended that despite
// intent most fair, the peasants: they dissent!
// \
// The wizards thought it boon to speak in verse
// but overestimate the peasant's skill;
// 'there is no point to it' the peasants thought.
// 'What cruel poetry they thrust on us.'
// And so the peasants organized revolt.
// \
// They searched for mages speaking just in verse;
// they thought and thought and thought 'where could they be?'",
//     b"
// But long had passed; magicians marched away
// from cruel bitter thought and cursed man.
// \
// And so in lack of overlords but yet
// still wrought by curse the agriculturists
// admitted thought that life is not so bad.
// Despite new vocal eccentricities,
// their burdens lifted free of mages cruel.
// They thought that they were cursed, but in fact
// \
// the wizards had abandonded cruel thought,
// and left the peasants free of emperor.
// \
// And so their revolution had achieved
// a world where peasants had to speak in verse
// but answered not to any cruel lord
// for they had long since gone, with nothing left
// but a society that slowly learned
// restriction fosters creativity.",
// ];

// #[allow(dead_code)]
// static MILO_POEM_2: [&[u8]; 1] = [b"
//     was name les int marioooo. int luigi.!
//     was name2 mor int marioooo. int luigi.!
//     "];

#[macro_export]
macro_rules! ghidra_marker {
    ($reg:literal) => {
        unsafe {
            // let f = concat!("mov ", $reg, ",", $reg);
            std::arch::asm!(concat!("mov ", $reg, ",", $reg));
            // std::arch::asm!("mov {0}, {0}", inout("eax") _a);
        };
    };
}
