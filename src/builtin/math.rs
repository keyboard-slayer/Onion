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

use crate::onionret::OnionRet;
use std::process::exit;

pub fn add(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2 
    {
        eprintln!("+: Not enough arguments provided");
        exit(1);
    }

    let mut acc = args.get(0).unwrap().clone();
    let last_type = acc.clone().type_name();
    let mut modified_arg = args.clone();
    modified_arg.remove(0);

    for arg in modified_arg 
    {
        if last_type != arg.clone().type_name()
        {
            eprintln!("+: Incompatible types");
            exit(1);
        }

        match arg
        {
            OnionRet::Int(d) => {
                if let OnionRet::Int(accu) = acc
                {
                    acc = OnionRet::Int(accu + d);
                } 
                else 
                {
                    eprintln!("+ : Can't add int with {}", acc.clone().type_name());
                    exit(1)
                }
            }

            OnionRet::Str(s) => {
                if let OnionRet::Str(mut accu) = acc.clone()
                {
                    accu.push_str(s.as_str());
                    acc = OnionRet::Str(accu.clone());
                }
                else 
                {
                    eprintln!("+ : Can't add str with {}", acc.clone().type_name());
                    exit(1)
                }
            }

            _ => {
                eprintln!("+ : Invalid type");
                exit(1);
            }
        }
    }
    
    acc
}