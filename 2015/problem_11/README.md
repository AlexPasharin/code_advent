This problem can be solved quickly manually, without any programming, with just mathematical reasoning, that is why there is no code in this subfolder.

# Part 1

--- Day 11: Corporate Policy ---
Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

- Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
- Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
- Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

For example:

- hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
- abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
- abbcegjk fails the third requirement, because it only has one double letter (bb).
- The next password after abcdefgh is abcdffaa.
- The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?

Puzzle input: `vzbxkghb`.

## Part 1 Solution

First note that the given input does not containt forbidden chars (i, l, or o). In case it would the first step would be to get rid of them in the following way described in the next paragraph.

Locate the first forbidden char (from the left), let n be its index. It is clear that incrementing letters from the right up to but not including the index n will always produce a password which still contains a forbidden character. Only when we start to increment the char in position n for the first time, we will get rid of the forbbiden character, at that point the password would be of the form (indices 0 to (n-1) same as in original password)(next char in position n)(letter 'a' in all subsequent positions). It is clear that this password does not contain foribidden characters, so we can switch to that password as a starting point. Hence, if in the following considerations we will always avoid forbidden characters, we can assume that the original password also did not contain forbidden characters.

Next notice that the input string does not contain any dublicate subsequent letters of the form `aa` or a substring of the form `abc`, where subsequent letters differ by exactly +1 position in the alphabet. From now on we will operate on the assumption original password does not have any desired combinations. We will come back to this assumption later.

Now imagine letters from the right start to increase, one index at a time. Since original password did not contain any of desired combinations, the only way they can appear is from letters changing during increment. It is easy to realise that the smallest string that satisfies requirements is of the form `aabcc`, where `a`, `b`, `c` are subsequent letters of the alphabet (not literal letters a, b, c in this context!). This string has 5 characters. This implies that while we are rotating at most 3 last letters, we cannot arrive at this form (because if it appears at any time before we touched 4th and 5th letters from the right, that would contradict the assumption that original password did not contain consequent dublicates). The first time we can arrive at it is when we are rotating the 4th (from the right) letter `a`, so that it coincises with the 5th one `a'`(on the right), assuming this letter is not literal letters y or z. This is because there must be `b`and `c`, which come after `a`. If 4th letter from the right is smaller than 5th, we can achieve this by just rotating 4th, until it becomes the same as 5th, assuming none of the two next letters in the alphabet (`b` and `c` above) are forbidden characters. In case one of them is forbidden, or 4th letter is bigger than 5th, we just continue incrementing until 5th letter starts to rotate, in which case we rotate it to the next letter, so that `b` and `c` above are not forbidden. The only case in which this will not work is when the next letter for 5th position is at least literal y.

Luckily these observations are enough to solve part 1. 4th letter from the right is literal k and 5th letter from the right is literal x, which is bigger than k. Appplying previous considerations we see that the solution for part 1 is the string

```
vzbxxyzz
```

While this is enough to solve the puzzle for given input, it works only under several assumptions, mentioned above. If password would already contain some of the desired subsequences (but not all of them), one would need to make adjustments to arguments above, change or extend them, depending on the position of those subsequences (closer to the beginning to the string, less work you need to do). At the moment I will not come up with general universal solution strategy for all cases. Also, if we would need to increase 5th letter so that it will become literal y or z, the solution above would not work and we will have to keep incrementing and come up with new arguments.

# Part 2

Santa's password expired again. What's the next one?

## Part 2 Solution

Input is now the string we get by increamenting solution of part 1 `vzbxxyzz` one step, which is the string `vzbxxzaa`.

This input does contain even two non-overlapping dublicate letters, but that does not actually make solution much easier, since they are located closer to the end of the string, one immediately in the end. It is clear that incrementing only portion `zaa` will never give us the three consequent letters of the alphabet, so will not arrive to the solution at least until we start touching the 4th letter from the end, at which point string is `vzbxyaaa`. Incrementing the last letter again will give us always a string with only one dublicate letter and after that one arrives at the string `vzbxyaba`, which now do not contain any desired sequences, so we can attempt to apply to it the observations we made in the part 1 solution. Unfortunately the same trick does not work anymore, since 4th letter from the end is bigger than the 5th letter from the end, so ending of the form `aabcc` that would give us solution immediately without touching the 5th letter is not possible anymore. That means we need to start rotating the 5th letter as well, which gives us `vzcaaaaa`. Rotating just 2 last letters might give us the increasing sequence of 3 subsequent letters (on the last 3 positions), but that will destroy any hope of having second dublicate, so we need to touch the 3rd letter as well. Thus we arrive at `vzcaabaa`. Again, considering only 2 last letters rotations, the only ways to arrive at 3 subsequent letters would be to have literal c after b, which leads us to `vzcaabca`. This is almost the solution - next increment `vzcaabcb` is not a solution yet, but `vzcaabcc` is. This is solution to part 2.

### Notes on programming solutions

If one wants to write a program that solves the puzzle for an arbitrary input, there are at least two ways. One is the obvious brute-force, just incrementing string until solution is found, but it's not very interesting. The second is to generalize the considerations above into a set of rules which would cover all cases and to turn it into an actual code. I might do it later at some point, but for now I am satisfied with "no code, just logic" solution presented above. It might not cover all possible inputs, but it contains enough directions to come up with general solution.
