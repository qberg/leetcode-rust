from collections import Counter

class Solution:
    def largestPalindromic(self, num: str) -> str:
        digit_count = Counter(num)
        res = ''.join(digit_count[c]//2 * c for c in '9876543210').lstrip('0')
        mid = max(digit_count[c]%2 * c for c in digit_count)
        return (res+mid+res[::-1]) or '0'











