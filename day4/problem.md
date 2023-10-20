
Context:

Camp space -> Sections (id)
Elf -> set of sections assigned

Overlap btw sections of elves

------------PART1---------------------

Input: big list of the section assignments for each pair

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6.

Problem: In how many assignment pairs does one range fully contain the other?



demo: 
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
For the first few pairs, this list means:
    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers.

