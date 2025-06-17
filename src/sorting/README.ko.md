## 정렬 알고리즘 (Sort Algorithms)

### [보고 정렬 (Bogo-sort)](./bogo_sort.rs)
![alt text][bogo-image]

[위키백과][bogo-wiki]에서: 컴퓨터 과학에서 보고 정렬은 생성 및 테스트 패러다임에 기반한 정렬 알고리즘입니다. 함수는 정렬된 것을 찾을 때까지 입력의 순열을 연속적으로 생성합니다. 정렬에는 유용하다고 간주되지 않지만, 비효율적인 알고리즘과 대조하여 교육 목적으로 사용될 수 있습니다.

__속성__
* 최악의 경우 성능 (무작위 버전에서는 무제한)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O((n+1)!)


### [버블 정렬 (Bubble)](./bubble_sort.rs)
![alt text][bubble-image]

[위키백과][bubble-wiki]에서: 버블 정렬은 때때로 가라앉는 정렬이라고도 하며, 정렬할 목록을 반복적으로 통과하면서 인접한 항목 쌍을 비교하고 잘못된 순서이면 교환하는 간단한 정렬 알고리즘입니다. 교환이 더 이상 필요하지 않을 때까지 목록을 통과하는 과정이 반복되며, 이는 목록이 정렬되었음을 나타냅니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O(n^2)

###### [toptal에서 알고리즘 동작 보기][bubble-toptal]



### [칵테일 셰이커 정렬 (Cocktail-Shaker)](./cocktail_shaker_sort.rs)
![alt text][shaker-image]

[위키백과][shaker-wiki]에서: 칵테일 셰이커 정렬은 양방향 버블 정렬, 칵테일 정렬, 셰이커 정렬(선택 정렬의 변형을 나타낼 수도 있음), 리플 정렬, 셔플 정렬 또는 셔틀 정렬이라고도 하며, 버블 정렬의 확장입니다. 이 알고리즘은 양방향으로 작동하여 버블 정렬을 확장합니다. 목록의 시작 부분으로 항목을 더 빨리 이동하여 버블 정렬을 개선하지만, 성능 향상은 미미합니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O(n^2)



### [빗질 정렬 (Comb-sort)](./comb_sort.rs)
![comb sort][comb-sort]

[위키백과][comb-sort-wiki]에서: 빗질 정렬은 비교적 간단한 정렬 알고리즘이며 셸 정렬이 삽입 정렬을 개선하는 것과 같은 방식으로 버블 정렬을 개선합니다. 빗질 정렬의 기본 아이디어는 간격(비교되는 두 요소 간의 거리)이 1보다 훨씬 클 수 있다는 것입니다. 그리고 실제 `교환`을 수행하는 버블 정렬의 내부 루프는 교환된 요소 간의 간격이 `축소 계수 k: [n/k, n/k^2, ..., 1]` 단계로 줄어들도록 수정됩니다. 그리고 간격은 모든 루프에서 축소 계수로 나누어지고 간격이 1이 될 때까지 프로세스가 반복됩니다. 이 시점에서 빗질 정렬은 목록이 완전히 정렬될 때까지 간격 1을 계속 사용합니다. 따라서 정렬의 최종 단계는 버블 정렬과 동일하지만, 이번에는 대부분의 거북이가 처리되었으므로 버블 정렬이 효율적입니다. 그리고 축소 계수는 빗질 정렬의 효율성에 큰 영향을 미치며 `k=1.3`이 이상적인 값으로 제안되었습니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n log n)
* 평균적인 경우 성능 O(n^2/2^p)

여기서 `p`는 증분 횟수입니다.



### [계수 정렬 (Counting)](./counting_sort.rs)

[위키백과][counting-wiki]에서: 컴퓨터 과학에서 계수 정렬은 작은 정수인 키에 따라 객체 모음을 정렬하는 알고리즘입니다. 즉, 정수 정렬 알고리즘입니다. 각 고유 키 값을 가진 객체 수를 계산하고 해당 개수에 대한 산술 연산을 사용하여 출력 시퀀스에서 각 키 값의 위치를 결정하는 방식으로 작동합니다. 실행 시간은 항목 수와 최대 키 값과 최소 키 값 간의 차이에 선형적이므로 키의 변형이 항목 수보다 훨씬 크지 않은 경우에만 직접 사용하기에 적합합니다. 그러나 더 큰 키를 더 효율적으로 처리할 수 있는 다른 정렬 알고리즘인 기수 정렬에서 서브루틴으로 자주 사용됩니다.

__속성__
* 최악의 경우 성능 O(n+k)
* 최상의 경우 성능 O(n+k)
* 평균적인 경우 성능 O(n+k),

여기서 n은 정렬할 정수 수이고 k는 목록에서 가장 큰 정수와 가장 작은 정수 간의 차이입니다.



### [삽입 정렬 (Insertion)](./insertion_sort.rs)
![alt text][insertion-image]

[위키백과][insertion-wiki]에서: 삽입 정렬은 한 번에 하나씩 항목을 정렬하여 최종 정렬된 배열(또는 목록)을 구축하는 간단한 정렬 알고리즘입니다. 퀵 정렬, 힙 정렬 또는 병합 정렬과 같은 고급 알고리즘보다 큰 목록에서는 훨씬 덜 효율적입니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O(n^2)

###### [toptal에서 알고리즘 동작 보기][insertion-toptal]


### [놈 정렬 (Gnome)](./gnome_sort.rs)
![alt text][gnome-image]

[위키백과][gnome-wiki]에서: 놈 정렬은 한 번에 하나의 항목으로 작동하지만 버블 정렬과 유사하게 일련의 교환을 통해 항목을 적절한 위치로 가져온다는 점에서 삽입 정렬과 유사한 정렬 알고리즘입니다. 중첩 루프가 필요 없이 개념적으로 간단합니다. 평균 실행 시간은 O(n^2)이지만 목록이 초기에 거의 정렬되어 있으면 O(n)에 가까워지는 경향이 있습니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O(n^2)



### [병합 정렬 (Merge)](./merge_sort.rs)
![alt text][merge-image]

[위키백과][merge-wiki]에서: 컴퓨터 과학에서 병합 정렬(또는 합병 정렬)은 효율적이고 범용적이며 비교 기반 정렬 알고리즘입니다. 대부분의 구현은 안정적인 정렬을 생성합니다. 즉, 구현은 정렬된 출력에서 동일한 요소의 입력 순서를 유지합니다. 병합 정렬은 1945년 존 폰 노이만이 발명한 분할 정복 알고리즘입니다.

__속성__
* 최악의 경우 성능 O(n log n)
* 최상의 경우 성능 O(n log n)
* 평균적인 경우 성능 O(n log n)


###### [toptal에서 알고리즘 동작 보기][merge-toptal]

### [홀짝 정렬 (Odd-even)](./odd_even_sort.rs)
![alt text][odd-even-image]

[위키백과][odd-even-wiki]에서: 컴퓨팅에서 홀짝 정렬 또는 홀짝 전치 정렬(벽돌 정렬 또는 패리티 정렬이라고도 함)은 로컬 상호 연결이 있는 병렬 프로세서에서 사용하기 위해 원래 개발된 비교적 간단한 정렬 알고리즘입니다. 많은 특성을 공유하는 버블 정렬과 관련된 비교 정렬입니다. 목록에서 인접한 요소의 모든 홀수/짝수 인덱스 쌍을 비교하고 쌍이 잘못된 순서이면(첫 번째가 두 번째보다 큼) 요소를 전환하는 방식으로 작동합니다. 다음 단계에서는 짝수/홀수 인덱스 쌍(인접 요소)에 대해 이 작업을 반복합니다. 그런 다음 목록이 정렬될 때까지 홀수/짝수 단계와 짝수/홀수 단계를 번갈아 수행합니다.

참고: 구현은 단일 프로세서 시스템용 알고리즘을 조정한 것이지만 원래 알고리즘은 여러 프로세서에서 동시에 실행되도록 고안되었습니다.
__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n)
* 평균적인 경우 성능 O(n^2)


### [팬케이크 정렬 (Pancake)](./pancake_sort.rs)
![alt text][pancake-image]

[위키백과][pancake-wiki]에서: 모든 정렬 방법에는 비교할 요소 쌍이 필요합니다. 전통적인 정렬 문제의 경우 일반적으로 연구되는 문제는 목록을 정렬하는 데 필요한 비교 횟수를 최소화하는 것입니다. 두 요소를 교환하는 것과 같은 실제 작업 수는 관련이 없습니다. 반대로 팬케이크 정렬 문제의 경우 허용되는 유일한 작업이 시퀀스의 일부 접두사 요소 반전인 작업 수를 최소화하는 것이 목표입니다. 이제 비교 횟수는 관련이 없습니다.


### [퀵 정렬 (Quick)](./quick_sort.rs)
![alt text][quick-image]

[위키백과][quick-wiki]에서: 퀵 정렬(때때로 파티션 교환 정렬이라고도 함)은 배열 요소를 순서대로 배치하는 체계적인 방법을 제공하는 효율적인 정렬 알고리즘입니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n log n) 또는 세 방향 파티션의 경우 O(n)
* 평균적인 경우 성능 O(n log n)

###### [toptal에서 알고리즘 동작 보기][quick-toptal]

### [기수 정렬 (Radix)](./radix_sort.rs)
![alt text][radix-image]

[위키백과][radix-wiki]에서: 기수 정렬은 비교하지 않는 정렬 알고리즘입니다. 기수에 따라 요소를 버킷으로 만들고 배포하여 비교를 피합니다. 유효 숫자가 두 개 이상인 요소의 경우 모든 숫자가 고려될 때까지 이전 단계의 순서를 유지하면서 각 숫자에 대해 이 버킷팅 프로세스를 반복합니다.

__속성__
* 최악의 경우 성능 O(w*n)

여기서 w는 각 키를 저장하는 데 필요한 비트 수입니다.

### [선택 정렬 (Selection)](./selection_sort.rs)
![alt text][selection-image]

[위키백과][selection-wiki]에서: 이 알고리즘은 입력 목록을 두 부분으로 나눕니다. 이미 정렬된 항목의 하위 목록은 목록의 앞(왼쪽)에서 왼쪽에서 오른쪽으로 빌드되고, 정렬해야 할 나머지 항목의 하위 목록은 목록의 나머지 부분을 차지합니다. 처음에는 정렬된 하위 목록이 비어 있고 정렬되지 않은 하위 목록은 전체 입력 목록입니다. 이 알고리즘은 정렬되지 않은 하위 목록에서 가장 작은(또는 정렬 순서에 따라 가장 큰) 요소를 찾고, 가장 왼쪽에 있는 정렬되지 않은 요소와 교환(교환)하고(정렬된 순서로 배치) 하위 목록 경계를 오른쪽으로 한 요소 이동하여 진행합니다.

__속성__
* 최악의 경우 성능 O(n^2)
* 최상의 경우 성능 O(n^2)
* 평균적인 경우 성능 O(n^2)

###### [toptal에서 알고리즘 동작 보기][selection-toptal]

### [셸 정렬 (Shell)](./shell_sort.rs)
![alt text][shell-image]

[위키백과][shell-wiki]에서: 셸 정렬은 멀리 떨어져 있는 항목을 교환할 수 있도록 하는 삽입 정렬의 일반화입니다. 아이디어는 요소 목록을 정렬하여 어디에서나 시작하여 모든 n번째 요소를 고려하면 정렬된 목록이 제공되도록 하는 것입니다. 이러한 목록을 h-정렬되었다고 합니다. 동등하게, 각각 개별적으로 정렬된 h개의 인터리브된 목록으로 생각할 수 있습니다.

__속성__
* 최악의 경우 성능 O(nlog2 2n)
* 최상의 경우 성능 O(n log n)
* 평균적인 경우 성능은 간격 시퀀스에 따라 다름

###### [toptal에서 알고리즘 동작 보기][shell-toptal]

### [스투지 정렬 (Stooge)](./stooge_sort.rs)
![alt text][stooge-image]

[위키백과][stooge-wiki]에서: 스투지 정렬은 재귀 정렬 알고리즘입니다. O(n^(log 3 / log 1.5)) = O(n^2.7095...)라는 매우 나쁜 시간 복잡도로 유명합니다. 따라서 알고리즘의 실행 시간은 합리적인 정렬 알고리즘에 비해 느리며, 상당히 비효율적인 정렬의 전형적인 예인 버블 정렬보다 느립니다. 그러나 느린 정렬보다는 효율적입니다. 이름은 삼총사(The Three Stooges)에서 유래했습니다.

__속성__
* 최악의 경우 성능 O(n^(log(3) / log(1.5)))

### [팀 정렬 (Tim)](./tim_sort.rs)
![alt text][tim-image]

[위키백과][tim-wiki]에서: 팀 정렬은 병합 정렬과 삽입 정렬에서 파생된 하이브리드 안정 정렬 알고리즘으로, 다양한 종류의 실제 데이터에서 잘 수행되도록 설계되었습니다. 2002년 팀 피터스(Tim Peters)가 파이썬 프로그래밍 언어에서 사용하기 위해 구현했습니다. 이 알고리즘은 이미 정렬된 데이터의 하위 시퀀스(실행)를 찾아 나머지 부분을 보다 효율적으로 정렬하는 데 사용합니다. 특정 기준이 충족될 때까지 실행을 병합하여 이 작업을 수행합니다. 팀 정렬은 버전 2.3부터 파이썬의 표준 정렬 알고리즘이었습니다. 또한 Java SE 7, Android 플랫폼, GNU Octave, V8, Swift 및 Rust에서 기본형이 아닌 유형의 배열을 정렬하는 데 사용됩니다.

__속성__
* 최악의 경우 성능 O(최대 요소 크기(ms))
* 최상의 경우 성능 O(최대 요소 크기(ms))

### [수면 정렬 (Sleep)](./sleep_sort.rs)
![alt text][sleep-image]

[위키백과][bucket-sort-wiki]에서: 이것은 원래 메시지 보드 4chan에 게시된 아이디어로, 버킷 정렬의 버킷을 메모리 공간 대신 시간으로 대체했습니다.
실제로 "모든 요소의 최댓값 x 단위 시간만큼 절전"으로 정렬하는 것이 가능합니다. 이것이 유용할 수 있는 유일한 경우는 예제입니다.

### [인내 정렬 (Patience)](./patience_sort.rs)
[patience-video]


[위키백과][patience-sort-wiki]에서: 이 알고리즘의 이름은 인내 카드 게임의 단순화된 변형에서 유래했습니다. 게임은 섞인 카드 덱으로 시작합니다. 카드는 다음 규칙에 따라 테이블의 파일 시퀀스로 하나씩 처리됩니다.

1. 처음에는 파일이 없습니다. 처리된 첫 번째 카드는 단일 카드로 구성된 새 파일을 형성합니다.
2. 각 후속 카드는 맨 위 카드의 값이 새 카드의 값보다 크거나 같은 가장 왼쪽에 있는 기존 파일 또는 기존 파일의 오른쪽에 배치되어 새 파일을 형성합니다.
3. 처리할 카드가 더 이상 없으면 게임이 종료됩니다.

이 카드 게임은 다음과 같이 2단계 정렬 알고리즘으로 전환됩니다. 완전히 정렬된 도메인의 n개 요소 배열이 주어지면 이 배열을 카드 모음으로 간주하고 인내 정렬 게임을 시뮬레이션합니다. 게임이 끝나면 보이는 최소 카드를 반복적으로 선택하여 정렬된 시퀀스를 복구합니다. 즉, 각각 내부적으로 정렬된 p개 파일의 k-way 병합을 수행합니다.

__속성__
* 최악의 경우 성능 O(n log n)
* 최상의 경우 성능 O(n)

[bogo-wiki]: https://ko.wikipedia.org/wiki/보고_정렬
[bogo-image]: https://upload.wikimedia.org/wikipedia/commons/7/7b/Bogo_sort_animation.gif

[bubble-toptal]: https://www.toptal.com/developers/sorting-algorithms/bubble-sort
[bubble-wiki]: https://ko.wikipedia.org/wiki/버블_정렬
[bubble-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Bubblesort-edited-color.svg/220px-Bubblesort-edited-color.svg.png "버블 정렬"

[shaker-wiki]: https://ko.wikipedia.org/wiki/칵테일_셰이커_정렬
[shaker-image]: https://upload.wikimedia.org/wikipedia/commons/e/ef/Sorting_shaker_sort_anim.gif

[counting-wiki]: https://ko.wikipedia.org/wiki/계수_정렬

[insertion-toptal]: https://www.toptal.com/developers/sorting-algorithms/insertion-sort
[insertion-wiki]: https://ko.wikipedia.org/wiki/삽입_정렬
[insertion-image]: https://upload.wikimedia.org/wikipedia/commons/7/7e/Insertionsort-edited.png "삽입 정렬"

[gnome-wiki]: https://ko.wikipedia.org/wiki/놈_정렬
[gnome-image]: https://upload.wikimedia.org/wikipedia/commons/3/37/Sorting_gnomesort_anim.gif "놈 정렬"

[pancake-wiki]: https://en.wikipedia.org/wiki/Pancake_sorting  <!-- 팬케이크 정렬에 대한 한국어 위키백과 문서가 없습니다. -->
[pancake-image]: https://upload.wikimedia.org/wikipedia/commons/0/0f/Pancake_sort_operation.png

[quick-toptal]: https://www.toptal.com/developers/sorting-algorithms/quick-sort
[quick-wiki]: https://ko.wikipedia.org/wiki/퀵_정렬
[quick-image]: https://upload.wikimedia.org/wikipedia/commons/6/6a/Sorting_quicksort_anim.gif "퀵 정렬"

[merge-toptal]: https://www.toptal.com/developers/sorting-algorithms/merge-sort
[merge-wiki]: https://ko.wikipedia.org/wiki/병합_정렬
[merge-image]: https://upload.wikimedia.org/wikipedia/commons/c/cc/Merge-sort-example-300px.gif "병합 정렬"

[odd-even-image]: https://upload.wikimedia.org/wikipedia/commons/1/1b/Odd_even_sort_animation.gif
[odd-even-wiki]: https://en.wikipedia.org/wiki/Odd%E2%80%93even_sort <!-- 홀짝 정렬에 대한 한국어 위키백과 문서가 없습니다. -->

[radix-wiki]: https://ko.wikipedia.org/wiki/기수_정렬
[radix-image]: https://ds055uzetaobb.cloudfront.net/brioche/uploads/IEZs8xJML3-radixsort_ed.png?width=400 "기수 정렬"

[selection-toptal]: https://www.toptal.com/developers/sorting-algorithms/selection-sort
[selection-wiki]: https://ko.wikipedia.org/wiki/선택_정렬
[selection-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/b/b0/Selection_sort_animation.gif/250px-Selection_sort_animation.gif "선택 정렬"

[shell-toptal]: https://www.toptal.com/developers/sorting-algorithms/shell-sort
[shell-wiki]: https://ko.wikipedia.org/wiki/셸_정렬
[shell-image]: https://upload.wikimedia.org/wikipedia/commons/d/d8/Sorting_shellsort_anim.gif "셸 정렬"

[stooge-image]: https://upload.wikimedia.org/wikipedia/commons/f/f8/Sorting_stoogesort_anim.gif
[stooge-wiki]: https://en.wikipedia.org/wiki/Stooge_sort <!-- 스투지 정렬에 대한 한국어 위키백과 문서가 없습니다. -->

[tim-image]: https://thumbs.gfycat.com/BruisedFrigidBlackrhino-size_restricted.gif
[tim-wiki]: https://en.wikipedia.org/wiki/Timsort <!-- 팀 정렬에 대한 한국어 위키백과 문서가 없습니다. -->

[comb-sort]: https://upload.wikimedia.org/wikipedia/commons/4/46/Comb_sort_demo.gif
[comb-sort-wiki]: https://ko.wikipedia.org/wiki/빗질_정렬

[sleep-sort]: <no image>
[sleep-sort-wiki]: https://ja.wikipedia.org/wiki/スリープソート <!-- 수면 정렬에 대한 한국어 위키백과 문서가 없어 일본어 위키로 연결합니다. -->

[patience-sort-wiki]: https://en.wikipedia.org/wiki/Patience_sorting <!-- 인내 정렬에 대한 한국어 위키백과 문서가 없습니다. -->
[patience-video]: https://user-images.githubusercontent.com/67539676/212542208-d3f7a824-60d8-467c-8097-841945514ae9.mp4
