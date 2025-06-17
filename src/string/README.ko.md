## 문자열 알고리즘 (String Algorithms)

### [아호-코라식 알고리즘 (Aho-Corasick Algorithm)](./aho_corasick.rs)
[위키백과][aho-corasick-wiki]에서: 1975년 알프레드 V. 아호와 마가렛 J. 코라식이 발명한 문자열 검색 알고리즘입니다.[1] 입력 텍스트 내에서 유한한 문자열 집합("사전")의 요소를 찾는 일종의 사전 일치 알고리즘입니다. 모든 문자열을 동시에 일치시킵니다.

[aho-corasick-wiki]: https://ko.wikipedia.org/wiki/아호-코라식_알고리즘


### [버로우즈-휠러 변환 (Burrows-Wheeler transform)](./burrows_wheeler_transform.rs)
[위키백과][burrows-wheeler-wiki]에서: 버로우즈-휠러 변환(BWT, 블록 정렬 압축이라고도 함)은 문자열을 유사한 문자의 실행으로 재정렬합니다. 반복되는 문자의 실행이 있는 문자열은 전면 이동 변환 및 실행 길이 인코딩과 같은 기술로 압축하기 쉽기 때문에 압축에 유용합니다. 더 중요한 것은 첫 번째 원본 문자의 위치를 제외하고 추가 데이터를 저장할 필요 없이 변환이 가역적이라는 것입니다. 따라서 BWT는 텍스트 압축 알고리즘의 효율성을 향상시키는 "무료" 방법이며 약간의 추가 계산만 필요합니다.

__속성__
* 최악의 경우 성능 O(n)

[burrows-wheeler-wiki]: https://ko.wikipedia.org/wiki/버로우즈-휠러_변환


### [커누스-모리스-프랫 알고리즘 (Knuth Morris Pratt)](./knuth_morris_pratt.rs)
[위키백과][kmp-wiki]에서: 불일치가 발생하면 단어 자체가 다음 일치가 시작될 수 있는 위치를 결정하기에 충분한 정보를 포함하므로 이전에 일치한 문자를 다시 검사하지 않도록 하는 관찰을 사용하여 주 "텍스트 문자열" S 내에서 "단어" W의 발생을 검색합니다.
  커누스-모리스-프랫 검색은 W와 S의 길이에 대해 선형 시간으로 실행됩니다.

__속성__
* 경우 성능 O(s + w)
* 경우 공간 복잡도 O(w)

[kmp-wiki]: https://ko.wikipedia.org/wiki/커누스-모리스-프랫_알고리즘



### [매내커 알고리즘 (Manacher)](./manacher.rs)
[위키백과][manacher-wiki]에서: 문자열에서 가장 긴 회문을 선형 시간으로 찾습니다.

__속성__
* 최악의 경우 시간 복잡도 O(n)
* 최악의 경우 공간 복잡도 O(n)

[manacher-wiki]: https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm <!-- 매내커 알고리즘에 대한 한국어 위키백과 문서가 특정 섹션을 가리키기 어려워 영어 위키로 유지합니다. -->


### [라빈-카프 알고리즘 (Rabin Karp)](./rabin_karp.rs)
[위키백과][rabin-karp-wiki]에서: 리처드 M. 카프와 마이클 O. 라빈이 만든 문자열 검색 알고리즘으로, 해싱을 사용하여 텍스트에서 패턴 문자열과 정확히 일치하는 항목을 찾습니다.

[rabin-karp-wiki]: https://ko.wikipedia.org/wiki/라빈-카프_알고리즘


### [해밍 거리 (Hamming Distance)](./hamming_distance.rs)
[위키백과][hamming-distance-wiki]에서: 정보 이론에서 길이가 같은 두 문자열 간의 해밍 거리는 해당 기호가 다른 위치의 수입니다. 즉, 한 문자열을 다른 문자열로 변경하는 데 필요한 최소 대체 횟수 또는 한 문자열을 다른 문자열로 변환했을 수 있는 최소 오류 수를 측정합니다. 보다 일반적인 맥락에서 해밍 거리는 두 시퀀스 간의 편집 거리를 측정하기 위한 여러 문자열 메트릭 중 하나입니다. 미국의 수학자 리처드 해밍의 이름을 따서 명명되었습니다.

[run-length-encoding-wiki]: https://ko.wikipedia.org/wiki/실행_길이_부호화

### [실행 길이 인코딩 (Run Length Encoding)](./run_length_encoding.rs)
[위키백과][run-length-encoding-wiki]에서: 데이터 실행(동일한 데이터 값이 많은 연속 데이터 요소에서 발생하는 시퀀스)이 원래 실행이 아닌 단일 데이터 값과 개수로 저장되는 무손실 데이터 압축의 한 형태입니다.

[hamming-distance-wiki]: https://ko.wikipedia.org/wiki/해밍_거리
