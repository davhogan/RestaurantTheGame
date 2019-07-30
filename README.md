# RestaurantTheGame
Authored by David Hogan

This is the term project for CS 410 Rust Programming class at PSU.

This is a text based resource managenment game. The player will be in charge of managing a small restaurant.
The player will be responsible for hiring,firing and scheduling employees, setting prices for items and ordering products.
The goal of the game is to maintain a profit and keep their restaruant open. 

The game simulates one day at a time. Giving the player a summary of costs and gains from the day. The daily profit will be affected by an assortment of factors. The factors include the compentensy of the employees scheduled, the amount of customers for the day, the price of the food. The costs will be a fixed and based off of known values such as employee wages and cost of goods. After each day the player will be given a chance to make serval changes; such as hiring new employees, changing the price on the menu, changing their advertising approach as a few examples. This game is meant as an exercise in resource management style games.

An employee should have an age between 16-75, a rating from 1-10 and a postion identifier from 0-2.
To create a random employee use randEmployee():
new_emp = randEmployee();
assert!(new_emp.age > 16 && new_emp.age < 76);
assert!(new_emp.rating > 1 && new_emp.rating < 10);
assert!(new_emp.position >0 && new_emp.position < 2);

[License](LICENSE)
