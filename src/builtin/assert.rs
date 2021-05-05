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

use crate::reader::OnionRet;
use std::process::exit;

pub fn assert_scheme(wrapped: OnionRet) -> OnionRet 
{
    if let OnionRet::List(args) = wrapped 
    {
        if args.len() == 0
        {
            eprintln!("assert: No argument provided");
            exit(1);
        }
        if args.len() > 1
        {
            eprintln!("assert: Too much arguments");
            exit(1);
        }

        if let OnionRet::Bool(truth) = args.get(0).unwrap()
        {
            if !truth 
            {
                eprintln!("Assertion failed");
                exit(1);
            }
        }
    }

    OnionRet::Nil
}
