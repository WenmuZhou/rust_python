import Levenshtein
import time
import edit_distence_rust
print(dir(edit_distence_rust))

tic = time.time()
for i in range(270000):
    dis = edit_distence_rust.edit_distance('我的中国心1', '别人也是调用的底层C文件吧')
print('我的rust cython so:', time.time()-tic, dis)

import Levenshtein
tic = time.time()
for i in range(270000):
    dis = Levenshtein.distance('我的中国心1', '别人也是调用的底层C文件吧')
print('别人的库', time.time()-tic, dis)


class Solution:
    def minDistance(self, word1, word2):

        l1 = len(word1) + 1
        l2 = len(word2) + 1
        if l2 > l1:
            return self.minDistance(word2, word1)
        m = [0]*l2  # 遍历到底i行时m[i]表示s1[:i-1]替换为s2[:j-1]的编辑距离
        for i in range(1, l2):
            m[i] = i
        p = 0  # 用于存储上一行左上角的值
        for i in range(1, l1):
            p = m[0]
            m[0] = i
            for j in range(1, l2):
                tmp = m[j]  # 先将上一行i处的结果存起来
                m[j] = p if word1[i-1] == word2[j -
                                                1] else min(m[j-1] + 1, m[j] + 1, p + 1)
                p = tmp
        return m[l2-1]


s = Solution()
d = 0
tic = time.time()
for i in range(270000):
    d = s.minDistance('我的中国心', '别人也是调用的底层C文件吧')
print('自己的py实现', time.time()-tic, d)
