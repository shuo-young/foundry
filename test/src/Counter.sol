// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;
    // C c = new C();

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
        // c.a();
    }
}

// contract C {
//     uint256 b;
//     function a() public {
//         b += 1;
//     }
// }
