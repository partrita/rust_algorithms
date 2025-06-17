## 검색 알고리즘 (Search Algorithms)

### [선형 검색 (Linear)](./linear_search.rs)
![alt text][linear-image]

[위키백과][linear-wiki]에서: 선형 검색 또는 순차 검색은 목록 내에서 대상 값을 찾는 방법입니다. 일치하는 항목을 찾거나 모든 요소를 검색할 때까지 목록의 각 요소를 순차적으로 확인합니다.
  선형 검색은 최악의 경우 선형 시간으로 실행되며, 목록의 길이를 n이라고 할 때 최대 n번의 비교를 수행합니다.

__속성__
* 최악의 경우 성능 O(n)
* 최상의 경우 성능 O(1)
* 평균적인 경우 성능 O(n)
* 최악의 경우 공간 복잡도 O(1) 반복

### [이진 검색 (Binary)](./binary_search.rs)
![alt text][binary-image]

[위키백과][binary-wiki]에서: 이진 검색은 반 간격 검색 또는 로그 검색이라고도 하며, 정렬된 배열 내에서 대상 값의 위치를 찾는 검색 알고리즘입니다. 배열의 중간 요소와 대상 값을 비교합니다. 같지 않으면 대상이 있을 수 없는 절반을 제거하고 성공할 때까지 나머지 절반에서 검색을 계속합니다.

__속성__
* 최악의 경우 성능 O(log n)
* 최상의 경우 성능 O(1)
* 평균적인 경우 성능 O(log n)
* 최악의 경우 공간 복잡도 O(1)

### [지수 검색 (Exponential)](./exponential_search.rs)
![alt text][exponential-image]

[위키백과][exponential-wiki]에서: 지수 검색을 사용하면 정렬된 무한 목록에서 지정된 입력 값(검색 "키")을 검색할 수 있습니다. 이 알고리즘은 두 단계로 구성됩니다. 첫 번째 단계에서는 검색 키가 목록에 있는 경우 해당 키가 있을 범위를 결정합니다. 두 번째 단계에서는 이 범위에서 이진 검색을 수행합니다. 첫 번째 단계에서는 목록이 오름차순으로 정렬되어 있다고 가정하고, 알고리즘은 값 2^j가 검색 키보다 큰 첫 번째 지수 j를 찾습니다. 이 값 2^j는 이진 검색의 상한이 되고 이전 2의 거듭제곱인 2^(j - 1)은 이진 검색의 하한이 됩니다.

__속성__
* 최악의 경우 성능 O(log i)
* 최상의 경우 성능 O(1)
* 평균적인 경우 성능 O(log i)
* 최악의 경우 공간 복잡도 O(1)

### [점프 검색 (Jump)](./jump_search.rs)
![alt text][jump-image]

[위키백과][jump-wiki]에서: 컴퓨터 과학에서 점프 검색 또는 블록 검색은 정렬된 목록에 대한 검색 알고리즘을 나타냅니다. 먼저 k ∈ N이고 m이 블록 크기인 모든 항목 L(km)을 확인하여 검색 키보다 큰 항목을 찾을 때까지 작동합니다. 목록에서 검색 키의 정확한 위치를 찾기 위해 하위 목록 L[(k-1)m, km]에서 선형 검색을 수행합니다.

__속성__
* 최악의 경우 성능 O(√n)
* 최상의 경우 성능 O(1)
* 평균적인 경우 성능 O(√n)
* 최악의 경우 공간 복잡도 O(1)

### [피보나치 검색 (Fibonacci)](./fibonacci_search.rs)

[위키백과][fibonacci-wiki]에서: 컴퓨터 과학에서 피보나치 검색 기법은 피보나치 수의 도움으로 가능한 위치를 좁히는 분할 정복 알고리즘을 사용하여 정렬된 배열을 검색하는 방법입니다. 정렬된 배열을 두 개의 동일한 크기의 부분으로 나누고 그중 하나를 추가로 검사하는 이진 검색과 비교하여 피보나치 검색은 배열을 연속적인 피보나치 수인 크기를 갖는 두 부분으로 나눕니다.

__속성__
* 최악의 경우 성능 O(log n)
* 최상의 경우 성능 O(1)
* 평균적인 경우 성능 O(log n)
* 최악의 경우 공간 복잡도 O(1)

[linear-wiki]: https://ko.wikipedia.org/wiki/선형_검색
[linear-image]: http://www.tutorialspoint.com/data_structures_algorithms/images/linear_search.gif

[binary-wiki]: https://ko.wikipedia.org/wiki/이진_검색_알고리즘
[binary-image]: https://upload.wikimedia.org/wikipedia/commons/f/f7/Binary_search_into_array.png

[exponential-wiki]: https://ko.wikipedia.org/wiki/지수_검색
[exponential-image]: https://upload.wikimedia.org/wikipedia/commons/4/45/Exponential_search.svg

[jump-wiki]: https://ko.wikipedia.org/wiki/점프_검색
[jump-image]: https://static.studytonight.com/data-structures/images/Jump%20Search%20technique.PNG

[fibonacci-wiki]: https://ko.wikipedia.org/wiki/피보나치_검색
