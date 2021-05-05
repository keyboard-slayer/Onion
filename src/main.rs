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

pub mod reader;
pub mod builtin;
mod evaluator;

use std::env::args;
use std::process::exit;

fn main() 
{
    if args().len() == 1
    {
        eprintln!("no file specified\n");
        exit(1);
    }

    let tokens = reader::read_str(format!("(load-file \"{}\")", args().nth(1).unwrap()));
    evaluator::eval(tokens);
}
