pub mod errors;
pub mod merkle_tree;

use errors::UserInterfaceErrors;
use merkle_tree::MerkleTree;

fn process_comands(line: String, tree: &mut MerkleTree) -> Result<(), UserInterfaceErrors> {
    let args: Vec<&str> = line.split_ascii_whitespace().collect();

    match args[0] {
        "--help" => {
            println!("  build - Usage: build <hash-1> <hash-2> ... <hash-n>");
            println!("  build-raw - Usage: build-raw <raw-text-1> <raw-text-2> ... <raw-text-n>");
            println!("  add-raw - Usage: add-raw raw-text");
            println!("  add - Usage: add 32-bytes-hash");
            println!("  verify - Usage: verify proof1 proof2 ... proofN seed index");
            println!("  proof - Usage: proof index");
            println!("  print - Usage: print");
        }
        "build" => {
            // Usage: build <hash-1> <hash-2> ... <hash-n>
            let hashes: Vec<&str> = Vec::from(&args[1..]);
            *tree = MerkleTree::build(hashes, false);
        }
        "build-raw" => {
            // Usage: build <raw-text-1> <raw-text-2> ... <raw-text-n>
            let hashes: Vec<&str> = Vec::from(&args[1..]);
            *tree = MerkleTree::build(hashes, true);
        }
        "add" => {
            // Usage: add hash
            if let Some(str) = args.get(1) {
                tree.add(str.to_string());
            } else {
                return Err(UserInterfaceErrors::NotEnoughArgumentsError(
                    "add hash".to_string(),
                ));
            }
        }
        "add-raw" => {
            // Usage: add-raw raw-text
            if let Some(str) = args.get(1) {
                tree.add_raw(str.to_string());
            } else {
                return Err(UserInterfaceErrors::NotEnoughArgumentsError(
                    "add-raw raw-text".to_string(),
                ));
            }
        }
        "verify" => {
            // Usage: verify proof1 proof2 ... proofN seed index
            if args.len() < 4 {
                return Err(UserInterfaceErrors::NotEnoughArgumentsError(
                    "verify proof1 proof2 ... proofN seed index".to_string(),
                ));
            }
            let mut proof = Vec::new();
            for item in args.iter().skip(1).take(tree.depth()) {
                proof.push((*item).to_string());
            }
            let leaf = args[1 + tree.depth()].to_string();

            match args[1 + tree.depth() + 1].to_string().parse() {
                Ok(mut index) => {
                    if tree.verify(proof, leaf, &mut index) {
                        println!("Proof has been verified");
                    } else {
                        println!("Proof has not been verified");
                    }
                }
                Err(e) => {
                    return Err(UserInterfaceErrors::NotCorrectTypeError(e));
                }
            }
        }
        "proof" => {
            // Usage: proof <index>
            if let Some(str) = args.get(1) {
                match str.parse::<usize>() {
                    Ok(mut index) => {
                        let response = tree.generate_proof(&mut index);
                        for hash in response {
                            print!("{hash} ");
                        }
                        println!();
                    }
                    Err(e) => {
                        return Err(UserInterfaceErrors::NotCorrectTypeError(e));
                    }
                }
            } else {
                return Err(UserInterfaceErrors::NotEnoughArgumentsError(
                    "proof <index>".to_string(),
                ));
            }
        }
        "print" => {
            tree.print();
        }
        _ => {
            println!("Command not recognized, type --help to see the available commands");
        }
    }
    Ok(())
}

fn main() {
    println!();
    println!("Welcome to this Merkle Tree simulator. Type --help to list the available commands");
    let mut tree = MerkleTree::new();
    loop {
        println!();

        let mut input_line = String::new();
        if let Ok(bytes_read) = std::io::stdin().read_line(&mut input_line) {
            if bytes_read <= 1 {
                return;
            }
        } else {
            println!("Could not receive from stdin");
            return;
        }

        let response = process_comands(input_line, &mut tree);
        if let Err(UserInterfaceErrors::NotCorrectTypeError(e)) = response {
            println!("{:?}", e);
        } else if let Err(UserInterfaceErrors::NotEnoughArgumentsError(usage)) = response {
            println!(
                "The amount of arguments is not the expected, usage: {}",
                usage
            );
        }
    }
}
