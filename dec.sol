pragma solidity ^0.8.0;

contract DecentralizedExchange {
    mapping(address => uint) public balances;

    event Bought(address indexed buyer, uint amount);
    event Sold(address indexed seller, uint amount);

    function buy() public payable {
        uint amount = msg.value / 1 ether; // Assuming 1 ETH = 1 Token
        require(amount > 0, "You need to send some Ether");
        balances[msg.sender] += amount;
        emit Bought(msg.sender, amount);
    }

    function sell(uint amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount * 1 ether); // Assuming 1 Token = 1 ETH
        emit Sold(msg.sender, amount);
    }
}
