class MedianFinder:

    def __init__(self):
        self.arr = []
        self.n = 0
        

    def addNum(self, num: int) -> None:
        self.arr.append(num)
        self.arr = sorted(self.arr)
        self.n += 1

    def findMedian(self) -> float:
        if self.n == 0:
            return None
        half = self.n / 2.0 - 1
        if self.n % 2 == 0:
            return (self.arr[int(half)] + self.arr[int(half + 1)]) / 2
        else:
            return self.arr[int(half + 0.5)]

        
        