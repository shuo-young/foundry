// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;
    C c = new C();

    function setNumber(uint256 newNumber) public {
        number = newNumber;
        c.a();
    }

    function increment() public {
        number++;
    }
}

contract C {
    uint256 b;
    function a() public {
        b += 1;
    }
}
