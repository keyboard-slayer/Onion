/*
 * Copyright (C) 2021 Jordan DALCQ (Keyboard-Slayer) & Contributors
 *
 * This file is part of Onion.
 *
 * Onion is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Onion is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Onion.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use crate::builtin::*;
use crate::reader::OnionRet;

use std::process::exit;
use std::collections::HashMap;

fn get_builtin() -> HashMap<String, OnionRet>
{
    let mut funcs: HashMap<String, OnionRet> = HashMap::new();
    funcs.insert(String::from("load-file"), OnionRet::Fn(load_file::load_file));

    funcs
}

fn execute(lst: Vec<OnionRet>) -> OnionRet 
{
    if let OnionRet::Fn(func) = lst.get(0).unwrap()
    {
        let mut arg = lst.clone();
        arg.remove(0);
        return func(OnionRet::List(arg.clone()));
    }
    else 
    {
        eprintln!("Expected a function");
        exit(1);
    }
}

pub fn eval(tokens: OnionRet) -> OnionRet
{
    let funcs = get_builtin();
    let mut return_value: OnionRet = OnionRet::Nil;

    match tokens 
    {
        OnionRet::List(l) => {
            let mut lst: Vec<OnionRet> = vec![];
            for tok in l.iter() 
            {
                lst.push(eval(tok.clone()));
            }
            
            return_value = execute(lst);
        }
        
        OnionRet::Symbol(s) => {
            if let Some(func) = funcs.get(&s) 
            {
                return_value = func.clone();
            }
            else 
            {
                eprintln!("{} undefined", s);
                exit(1);
            }
        }

        OnionRet::Str(s) => {
            return_value = OnionRet::Str(s);
        }

        _ => eprintln!("TODO !")
    }

    return_value
}