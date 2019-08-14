# RestaurantTheGame
Authored by David Hogan

This is the term project for CS 410 Rust Programming class at PSU.

This is a text based resource managenment game. The player will be in charge of managing a small restaurant.
The player will be responsible for hiring and firing employees, setting prices for items and ordering products.
The goal of the game is to maintain a profit and keep their restaruant open. 

When the game is started a player will be asked to input the name of the restaurant. The player will then be taken to the home screen where they can play the game. From the home screen a player can choose to look at the menu, fire or hire and employee, change the price or qulity of a menu item, order more inventory and simulate to the next day.

The player can hire employees from a list of potential employees. A list of employees that have random stats are placed in the list of potential employees. The list of potential employees is updated and new every in game week. A player can also fire an employee, this is accomplished by removing the chosen employee from the hired employees list.

When a player orders more inventory, their revenue is reduced by the cost of the item ordered times the amount ordered. If the player can't afford it then the order will not go through. If the player can afford it then their inventory of the chosen item is increased by the amount ordered.

The game simulates one day at a time. Giving the player a summary of costs and gains from the day. The daily profit will be affected by an assortment of factors. The factors include the the number of and overall total rating of the employees, the amount of customers for the day and the price of the food. 

The total rating of all the employees will be a factor in determining how many customers can be served in a day. The other factor is the total quality of all the items on the menu. The higher the total quality the more customers will be avliable for the day. This means that the total amount of customers served is the minimum value between the total rating of the employees and the number of customers derivied from the total menu quality. 

The customers will order a burger if they can afford it. A customer will also have a preference for whether or not they like fries and soda. If they like either item they will order it if they have enough cash to order it. A restaurant will have a new list of customers everyday new day.

The daily costs will be calculated by the total amount of employees mutliplied by their wage then multiplied by 8 (amount of hours worked in a day). After each day the player will be given a chance to make serval changes; such as hiring new employees, changing the price on the menu as a few examples. This game is meant as an exercise in resource management style games.


[License](LICENSE)
