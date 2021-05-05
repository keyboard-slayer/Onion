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

use crate::reader::{OnionRet, read_str};
use crate::evaluator::eval;
use std::process::exit;
use std::fs;

pub fn load_file(wrapped: OnionRet) -> OnionRet 
{
    let source_code: String;

    if let OnionRet::List(args) = wrapped {
        if args.len() == 0
        {
            eprintln!("load-file: No argument provided");
            exit(1);
        }

        if args.len() > 1 
        {
            eprintln!("load-file: Too much arguments");
            exit(1);
        }

        if let OnionRet::Str(path) =  args.get(0).unwrap()
        {
            if let Ok(file_content) = fs::read_to_string(path)
            {
                source_code = file_content;
            }
            else 
            {
                eprintln!("load-file: Couldn't read {}", path);
                exit(1);
            }

            return eval(read_str(source_code));
        }
        else 
        {
            eprintln!("load-file: Invalid argument type (Was expecting a string)");
            exit(1);
        }
    }

    OnionRet::Nil
}