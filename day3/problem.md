
# Loading Rucksacks with supplies

Each rucksack has two compartments: 1nd and 2nd
Both are filled with same amount of items.
Every item type is identified by a single lowercase or uppercase letter

-> Items of same type should be present in exactly one of the compartments

----------PART1-----------------------
Elf made a mess of just above rule

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

To help prioritize item rearrangement, every item type can be converted to a priority:
- Lowercase item types a through z have priorities 1 through 26.
- Uppercase item types A through Z have priorities 27 through 52.

Quest: Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

Solution: 
1. Find item shared in both compartment for each rucksack
2. Cast item into their priorities
3. Sum all the priorities

--------------PART2 ------------
Each of group of three rucksack will have a common item that identifies them being in the same group.
That is, if a group's badge is item type B, then all three rucksacks will have item type B somewhere inside, and at most two of the rucksacks will be carrying any other item type.

Quest: 
- Find the item type that corresponds to the badges of each three-Elf group. 
- What is the sum of the priorities of those item types?

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type.









