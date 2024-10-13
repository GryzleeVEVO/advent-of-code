<?php
// Part 1:
//  - Wires labeled a,b,c,...
//  - 16 bit signal
//  - Bitwise logic gates:
//      - NOT (NOT wire -> wire_out)
//      - AND (wire_1/const_1 AND wire_2/const_2 -> wire_out)
//      - OR (wire_1/const_1 OR wire_2/const_2 -> wire_out)
//      - LSHIFT (wire LSHIFT constant -> wire_out)
//      - RSHIFT (wire LSHIFT constant -> wire_out)
//      - NOTE: Gates don't output until all inputs are connected
//  - Numerical constants (constant -> wire)
//  - One source for input, possible multiple outputs
//  - Objective: What does the wire a output?
//
// Part 2: Now b is wired to the solution of part 1
//
// NOTE: & 0xffff means leave only 16 lower bits, since PHP's ints depend on
//          arch int size (so 64 bit on x86_64)
// TODO: Solution is super inelegant.
//          - The circuit resembles a graph
//          - Maybe creating a DT to store it and compute any value...
//          - ... but PHP is not the best fit

$f = file("input.txt", FILE_SKIP_EMPTY_LINES);
$wires = array();

while (!empty($f)) {
    foreach ($f as $idx => $line) {
        $matches = array();

        if (preg_match("/^([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // Constant
            $constant = intval($matches[1]);
            $wire_out = $matches[2];

            // Constants can always be resolved immediately
            $wires[$wire_out] = $constant;
            unset($f[$idx]);
        } else if (preg_match("/^([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // Constant
            $wire_in = $matches[1];
            $wire_out = $matches[2];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = $wires[$wire_in];
                unset($f[$idx]);
            }
        } else if (
            preg_match("/^NOT ([a-z]+) -> ([a-z]+)$/", $line, $matches)
        ) {
            // NOT
            $wire_in = $matches[1];
            $wire_out = $matches[2];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = (~$wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) AND ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // AND
            $wire_a = $matches[1];
            $wire_b = $matches[2];
            $wire_out = $matches[3];

            if (
                array_key_exists($wire_a, $wires)
                && array_key_exists($wire_b, $wires)
            ) {
                $wires[$wire_out] = ($wires[$wire_a] & $wires[$wire_b]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([0-9]+) AND ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // AND with a constant
            $constant = intval($matches[1]);
            $wire_in = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($constant & $wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) OR ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // OR
            $wire_a = $matches[1];
            $wire_b = $matches[2];
            $wire_out = $matches[3];

            if (
                array_key_exists($wire_a, $wires)
                && array_key_exists($wire_b, $wires)
            ) {
                $wires[$wire_out] = ($wires[$wire_a] | $wires[$wire_b]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([0-9]+) OR ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // OR with a constant
            $constant = intval($matches[1]);
            $wire_in = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($constant | $wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) LSHIFT ([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // LSHIFT
            $wire_in = $matches[1];
            $constant = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($wires[$wire_in] << $constant) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) RSHIFT ([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // RSHIFT
            $wire_in = $matches[1];
            $constant = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($wires[$wire_in] >> $constant) & 0xffff;
                unset($f[$idx]);
            }
        } else {
            print ("Unexpected line: " . $line);
            exit(1);
        }
    }
}

$sol_part_1 = $wires["a"];

// Should be 3176. First try weeeeeeeeeeee
print ("Part 1: " . $wires["a"] . "\n");


// Now let's do all of it again
$f = file("input.txt", FILE_SKIP_EMPTY_LINES);
$wires = array();

while (!empty($f)) {
    foreach ($f as $idx => $line) {
        $matches = array();

        if (preg_match("/^([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // Constant
            $constant = intval($matches[1]);
            $wire_out = $matches[2];

            if (strcmp($wire_out, "b") == 0) {
                // Let's make an exception for -> b
                $wires[$wire_out] = $sol_part_1;
            } else {
                // Constants can always be resolved immediately
                $wires[$wire_out] = $constant;
            }

            unset($f[$idx]);
        } else if (preg_match("/^([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // Constant
            $wire_in = $matches[1];
            $wire_out = $matches[2];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = $wires[$wire_in];
                unset($f[$idx]);
            }
        } else if (
            preg_match("/^NOT ([a-z]+) -> ([a-z]+)$/", $line, $matches)
        ) {
            // NOT
            $wire_in = $matches[1];
            $wire_out = $matches[2];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = (~$wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) AND ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // AND
            $wire_a = $matches[1];
            $wire_b = $matches[2];
            $wire_out = $matches[3];

            if (
                array_key_exists($wire_a, $wires)
                && array_key_exists($wire_b, $wires)
            ) {
                $wires[$wire_out] = ($wires[$wire_a] & $wires[$wire_b]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([0-9]+) AND ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // AND with a constant
            $constant = intval($matches[1]);
            $wire_in = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($constant & $wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) OR ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // OR
            $wire_a = $matches[1];
            $wire_b = $matches[2];
            $wire_out = $matches[3];

            if (
                array_key_exists($wire_a, $wires)
                && array_key_exists($wire_b, $wires)
            ) {
                $wires[$wire_out] = ($wires[$wire_a] | $wires[$wire_b]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([0-9]+) OR ([a-z]+) -> ([a-z]+)$/", $line, $matches)) {
            // OR with a constant
            $constant = intval($matches[1]);
            $wire_in = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($constant | $wires[$wire_in]) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) LSHIFT ([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // LSHIFT
            $wire_in = $matches[1];
            $constant = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($wires[$wire_in] << $constant) & 0xffff;
                unset($f[$idx]);
            }
        } else if (preg_match("/^([a-z]+) RSHIFT ([0-9]+) -> ([a-z]+)$/", $line, $matches)) {
            // RSHIFT
            $wire_in = $matches[1];
            $constant = $matches[2];
            $wire_out = $matches[3];

            if (array_key_exists($wire_in, $wires)) {
                $wires[$wire_out] = ($wires[$wire_in] >> $constant) & 0xffff;
                unset($f[$idx]);
            }
        } else {
            print ("Unexpected line: " . $line);
            exit(1);
        }
    }
}

// Should be 14710
print ("Part 2: " . $wires["a"] . "\n");