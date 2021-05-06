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

use std::process::exit;
use crate::onionret::OnionRet;

pub fn equal(args: Vec<OnionRet>) -> OnionRet 
{
    if args.len() < 2
    {
        eprintln!("=: Not enough argument provided");
        exit(1);
    }

    let first_arg = args.get(0).unwrap();
        
    for arg in args.clone()
    {
        if arg != first_arg.clone()
        {
            return OnionRet::Bool(false);
        }
    }

    OnionRet::Bool(true)
}

pub fn gt(args: Vec<OnionRet>) -> OnionRet 
{
    let mut return_value: bool = true;
    let mut last_value: OnionRet;
    let mut modified_arg = args.clone();

    if args.len() == 0
    {
        eprintln!(">: No argument provided");
        exit(1);
    }

    last_value = args.get(0).unwrap().clone();
    modified_arg.remove(0); 

    for arg in modified_arg
    {
        return_value &= last_value > arg;
        last_value = arg;
    }

    OnionRet::Bool(return_value)
}

pub fn lt(args: Vec<OnionRet>) -> OnionRet 
{
    let mut return_value: bool = true;
    let mut last_value: OnionRet;
    let mut modified_arg = args.clone();

    if args.len() == 0
    {
        eprintln!("<: No argument provided");
        exit(1);
    }

    last_value = args.get(0).unwrap().clone();
    modified_arg.remove(0); 

    for arg in modified_arg
    {
        return_value &= last_value < arg;
        last_value = arg;
    }

    OnionRet::Bool(return_value)
}