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

pub fn minus(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2 
    {
        eprintln!("-: Not enough arguments provided");
        exit(1);
    }

    let mut acc: i64 = 0;

    if let OnionRet::Int(d) = args.get(0).unwrap()
    {
        acc += d;
    }
    else 
    {
        eprintln!("-: Invalid argument type");
        exit(1);
    }

    let mut new_arg = args.clone();
    new_arg.remove(0);

    for arg in new_arg
    {
        if let OnionRet::Int(d) = arg 
        {
            acc -= d;
        }
        else 
        {
            eprintln!("-: Invalid argument type");
            exit(1);
        }
    }

    OnionRet::Int(acc)
}

pub fn mult(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2 
    {
        eprintln!("-: Not enough arguments provided");
        exit(1);
    }

    let mut acc: i64 = 1;
    let mut final_str: String = String::from("");

    for arg in args 
    {
        match arg 
        {
            OnionRet::Int(d) => acc *= d,
            OnionRet::Str(s) => {
                if !final_str.is_empty()
                {
                    eprintln!("*: Can only have on string");
                    exit(1);
                }
                else 
                {
                    final_str = s;
                }
            }
            _ => {
                eprintln!("*: Invalid argument type");
                exit(1);
            }
        }
    }

    if !final_str.is_empty()
    {
        OnionRet::Str(final_str.repeat(acc as usize))
    }
    else 
    {
        OnionRet::Int(acc)
    }
}

pub fn div(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2 
    {
        eprintln!("/: Not enough arguments provided");
        exit(1);
    }

    let mut acc: f64 = 0.0;

    if let OnionRet::Int(d) = args.get(0).unwrap()
    {
        acc += *d as f64;
    }
    else 
    {
        eprintln!("/: Invalid argument type");
        exit(1);
    }

    let mut new_arg = args.clone();
    new_arg.remove(0);

    for arg in new_arg
    {
        if let OnionRet::Int(d) = arg 
        {
            acc /= d as f64;
        }
        else 
        {
            eprintln!("/: Invalid argument type");
            exit(1);
        }
    }

    OnionRet::Float(acc)
}

pub fn full_div(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2 
    {
        eprintln!("/: Not enough arguments provided");
        exit(1);
    }

    let div_result = div(args);

    if let OnionRet::Float(f) = div_result 
    {
        OnionRet::Int(f as i64)
    }
    else 
    {
        panic!()
    }
}