
use std::fs;
use std::fs::File;
use std::path::{Path,PathBuf};
use std::io::{Error, ErrorKind, Result as IOResult, LineWriter, Write};

use super::types::{Gate, IOPin, Constant, GateType, Pin};

const CIRCUIT: &'static str = "circuit.txt";
const META_INFO: &'static str = "meta_info.txt";
const NEW_LINE: &'static [u8] = &['\n' as u8];

pub struct Converter<'a> {
    path: &'a Path,
    cap: usize,
}

impl<'a> Converter<'a> {

    fn join_path(&self, filename: &'static str) -> PathBuf {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(filename));
        pathbuf
    }

    fn open_file(&self, filename: &'static str) -> IOResult<LineWriter<File>> {
        let pathbuf = self.join_path(filename);
        let fpath = pathbuf.as_path();

        if fpath.exists() && !fpath.is_file() {
            return Err(Error::new(ErrorKind::AlreadyExists, format!("{} is a folder", fpath.display())))
        }

        let file = try!(File::create(self.join_path(filename).as_path()));
        let writer: LineWriter<File> = LineWriter::with_capacity(self.cap, file);
        Ok(writer)
    }

    pub fn new(path: &Path) -> IOResult<Converter> {
        Self::with_capacity(4 * 1024* 1024, path)  // 4 MB
    }

    pub fn with_capacity(cap: usize, path: &Path) -> IOResult<Converter> {
        if !path.is_dir(){
            if !path.exists() {
                try!(fs::create_dir(path));
            }else {
                return Err(Error::new(ErrorKind::NotFound, format!("{} isn't a folder", path.display())))
            }
        }
        Ok(Converter{
            path: path,
            cap: cap,
        })
    }

    pub fn buffering(&mut self, cap: usize) {
        self.cap = cap;
    }
    
    pub fn convert_circuit(&self, inputs: &Vec<IOPin>, gates: &Vec<Gate>) -> IOResult<()>{
        let mut writer: LineWriter<File> = try!(self.open_file(CIRCUIT));
        let (mut i, len) = (0, inputs.len());
        for input in inputs {
            try!(writer.write_fmt(format_args!("{}", input)));
            
            if i < len-1 {
                try!(writer.write_all(NEW_LINE));
                i += 1;
            }
        }

        let (mut i, len) = (0, gates.len());
        if len > 0{
            try!(writer.write_all(NEW_LINE));
        }

        for gate in gates {
            try!(writer.write_fmt(format_args!("{}", gate)));
            
            if i < len-1 {
                try!(writer.write_all(NEW_LINE));
                i += 1;
            }
        }
        writer.flush()
    }

    pub fn create_meta_info(&self, inputs: &Vec<IOPin>, gates: &Vec<Gate>, constant: Option<Constant>) -> IOResult<()>{
        let mut writer: LineWriter<File> = try!(self.open_file(META_INFO));
        try!(writer.write_fmt(format_args!("INPUTS = {}", inputs.len())));
        try!(writer.write_all(NEW_LINE));
        try!(writer.write_fmt(format_args!("GATES = {}", gates.len()+1)));
        try!(writer.write_all(NEW_LINE));
        try!(writer.write_fmt(format_args!("OUTPUTS = {}", count_outputs(gates))));
        match constant {
            Some(cons) => {
                try!(writer.write_all(NEW_LINE));
                try!(writer.write_fmt(format_args!("{}", cons)));
            },
            _ => (),
        };
        writer.flush()
    }

    pub fn replace_not_gates(&self, gates: &Vec<Gate>, cons: Option<Constant>) -> (Vec<Gate>, Option<Constant>){
        let mut constant: Option<Constant> = None;
        let mut not_free = Vec::new();
        for gate in gates{
            match gate.get_type() {
                GateType::NOT => {
                    let id: u64 = gate.id().into();
                    let mut xor = Gate::new(GateType::XOR, id);
                    for wire in gate {
                        xor.connect(*wire);
                    }
                    not_free.push(xor);
                    if constant.is_none() {
                         constant = Some(Constant::new());
                    }
                    constant.as_mut().unwrap().connect_to_gate(id, Pin::Right);
                },
                _ => not_free.push(gate.clone()),
            }
        }
        match (constant, cons) {
            (Some(mut c1), Some(c2)) => {
                c1.connect(c2);
                (not_free, Some(c1))
            }
            (Some(c), None) => (not_free, Some(c)),
            (None, Some(c)) => (not_free, Some(c)),
            _ => (not_free, None),
        }
    }
}

fn count_outputs(gates: &Vec<Gate>) -> u64{
    let mut ctr = 1;
    for gate in gates {
        for wire in gate {
            if wire.is_output() {
                ctr += 1;
            }
        }
    }
    ctr
}