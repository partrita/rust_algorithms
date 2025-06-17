### [B-트리 (B-Trees)](./b_tree.rs)

B-트리는 자가 균형을 맞추는 2-3 트리의 한 버전입니다. 디스크 읽기를 개선하는 데 사용되며 모든 트리 연산에 대해 O(log(n))의 복잡도를 가집니다. 특정 노드가 갖는 자식/키의 수는 해당 트리의 분기 계수/차수에 의해 결정됩니다.
B-트리는 항상 정렬된 키를 가집니다.

- 분기 계수(B) / 차수 (D):
  B = n인 경우, n <= 노드당 자식 수 < 2(n), n-1 <= 노드당 키 수 < 2(n) - 1

__속성__
* 모든 연산에 대한 최악/평균 성능 O(log n)
* 공간 복잡도 O(n)

__참고 자료:__
* [Busying Oneself with B-Trees](https://medium.com/basecs/busying-oneself-with-b-trees-78bbf10522e7)
* [Geeksforgeeks](https://www.geeksforgeeks.org/introduction-of-b-tree-2/)
* [Rust API Docs](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
* [Keon Algorithms](https://github.com/keon/algorithms)
* [MIT Open Course](https://www.youtube.com/watch?v=TOb1tuEZ2X4)

### [AVL 트리 (AVL Tree)](./avl_tree.rs)

AVL 트리는 자가 균형 이진 검색 트리입니다. 임의의 두 형제 노드의 높이 차이는 최대 1이어야 합니다. 이 속성을 유지하기 위해 삽입 또는 삭제 후 트리가 자체적으로 균형을 재조정할 수 있습니다.

__속성__
* 기본 연산에 대한 최악/평균 시간 복잡도: O(log n)
* 최악/평균 공간 복잡도: O(n)

__참고 자료:__
* [위키백과](https://ko.wikipedia.org/wiki/AVL_트리)
* Geeksforgeeks
([삽입](https://www.geeksforgeeks.org/avl-tree-set-1-insertion),
[삭제](https://www.geeksforgeeks.org/avl-tree-set-2-deletion))


### [이중 연결 리스트 (Doubly linked list)](./linked_list.rs)
![alt text][doubly-linked-list]

연결 리스트는 `선형` 자료 구조이며, 연결 리스트의 각 요소는 실제로 별도의 객체인 반면 모든 객체는 각 요소의 `참조 필드에 의해 함께 연결`됩니다. `이중 연결 리스트`에서 각 노드는 `다음` 노드 링크 외에 시퀀스의 `이전` 노드를 가리키는 두 번째 링크 필드를 포함합니다. 두 링크는 `next`와 `prev`라고 불릴 수 있습니다. 그리고 많은 최신 운영 체제는 활성 프로세스, 스레드 및 기타 동적 객체에 대한 참조를 유지하기 위해 이중 연결 리스트를 사용합니다.

__속성__
* 인덱싱 O(n)
* 삽입 O(1)
  * 시작 O(1)
  * 중간 (인덱싱 시간+O(1))
  * 끝 O(n)
* 삭제 O(1)
  * 시작 O(1)
  * 중간 (인덱싱 시간+O(1))
  * 끝 O(n)
* 검색 O(n)

__참고 자료:__
* [위키백과](https://ko.wikipedia.org/wiki/연결_리스트)
* [LeetCode](https://leetcode.com/explore/learn/card/linked-list/)
* [Brilliant](https://brilliant.org/wiki/linked-lists/)
* [Rust API Docs](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)


### [단일 연결 리스트를 사용한 스택 (Stack Using Singly Linked List)](./stack_using_singly_linked_list.rs)
![][stack]

위키백과에 따르면, 스택은 두 가지 주요 연산인 `Push`와 `Pop`을 사용하여 요소 모음 역할을 하는 추상 데이터 유형입니다.

__속성__
* Push O(1)
* Pop head.data O(1) tail.data O(n)
* Peek O(1)


__참고 자료:__
* [위키백과](https://ko.wikipedia.org/wiki/스택)
* [rust-unofficial](https://rust-unofficial.github.io/too-many-lists/index.html)
* [Stack Implementation and complexity](https://medium.com/@kaichimomose/stack-implementation-and-complexity-c176924e6a6b)



[doubly-linked-list]: https://upload.wikimedia.org/wikipedia/commons/thumb/5/5e/Doubly-linked-list.svg/610px-Doubly-linked-list.svg.png

[stack]: https://upload.wikimedia.org/wikipedia/commons/thumb/b/b4/Lifo_stack.png/700px-Lifo_stack.png
