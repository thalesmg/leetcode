from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        index = {}
        for i, n in enumerate(nums):
            if n in index:
                index[n].append(i)
            else:
                index[n] = [i]
        for n, js in index.items():
            # since the solution is unique, if n == target // 2 and
            # len(js) > 1, then ns is the solution
            if n == target // 2 and len(js) > 1:
                return js
            j = js[0]
            m = target - n
            # already checked for duplicate number
            if m == n:
                continue
            ks = index.get(m)
            if ks:
                return [j, ks[0]]
            else:
                continue
