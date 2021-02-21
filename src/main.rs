mod blob;
mod branch;
mod commit;
mod file;
mod index;
mod repo;
mod tree;
mod merge;
mod commandparser;
mod diff;
mod test;

fn main() {
    commandparser::read_command();
}

