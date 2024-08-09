---
title: Annotation Context
order: 800
---

## 개요

식별자("Class ID")를 인스턴스나 엔티티에 할당하는 모든 시각화는 Annotation을 사용하여 이점을 얻을 수 있습니다.
Annotation Context를 사용하면 주어진 클래스에 레이블과 색상을 연결하고, 그 클래스를 엔티티 전반에 재사용할 수 있습니다.

<!-- 예제 링크는 `latest`를 가리켜야 하지만 작성 시점에 샘플의 이름이 변경되었습니다 -->
이는 classification 알고리즘의 출력을 시각화하는 데 특히 유용합니다
([Detect and Track Objects](https://github.com/rerun-io/rerun/tree/main/examples/python/detect_and_track_objects) 예제에서 시연됨),
그러나 Rerun 기록 내에서 반복되는 분류의 모든 종류에 대해 더 일반적으로 사용할 수 있습니다.

<picture>
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/classids/7f881338f1970161f52a00f1ddd01d4dcccf8a46/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/classids/7f881338f1970161f52a00f1ddd01d4dcccf8a46/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/classids/7f881338f1970161f52a00f1ddd01d4dcccf8a46/1024w.png">
  <img src="https://static.rerun.io/classids/7f881338f1970161f52a00f1ddd01d4dcccf8a46/full.png" alt="다양한 추적 객체와 그 클래스 ID를 보여주는 뷰어 스크린샷">
</picture>



### 키포인트 및 키포인트 연결

Rerun은 클래스 *내부*에 키포인트를 정의할 수 있도록 합니다.
각 키포인트는 부모 클래스의 속성을 덮어쓸 수 있는 고유한 속성(색상, 레이블 등)을 정의할 수 있습니다.

키포인트의 일반적인 사용 예는 포즈 감지 내에서 스켈레톤의 관절을 Annotation 처리하는 것입니다.
이 경우, 전체 감지된 포즈/스켈레톤은 Class ID가 할당되고 각 관절은 Keypoint ID를 받습니다.

이(및 유사한) 사용 사례에 대해 더 도움을 주기 위해, Annotation 클래스 설명의 일환으로 키포인트 간의 연결을 정의할 수도 있습니다.
뷰어는 해당 클래스가 사용될 때 모든 연결된 키포인트에 대한 연결선을 그립니다.
레이블 및 색상과 마찬가지로, 이는 장면 내의 해당 클래스의 모든 인스턴스에서 동일한 연결 정보를 사용할 수 있게 해줍니다.

키포인트는 현재 2D 및 3D 포인트에만 적용 가능합니다.

<picture>
  <img src="https://static.rerun.io/keypoints/07b268032ab7cd26812de6b83e018b8ab55ed2f2/full.png" alt="3D 스켈레톤에 표시된 키포인트">
</picture>



### Annotation Context 로깅

Annotation Context는 일반적으로 [타임리스](timelines.md#timeless-data) 데이터로 로깅되지만 필요에 따라 시간이 지남에 따라 변경될 수 있습니다.

Annotation Context는 클래스가 스타일링되는 방식을 정의하는 클래스 설명 목록으로 정의됩니다
(선택적 키포인트 스타일 및 연결 포함).

Annotation Context는 다음과 함께 로깅됩니다:

* Python: 🐍[`rr.AnnotationContext`](https://ref.rerun.io/docs/python/stable/common/archetypes/#rerun.archetypes.AnnotationContext)
* Rust: 🦀[`rerun::AnnotationContext`](https://docs.rs/rerun/latest/rerun/archetypes/struct.AnnotationContext.html#)

snippet: tutorials/annotation-context


## 영향을 받는 엔티티

Class ID 구성 요소(및 선택적으로 Keypoint ID 구성 요소)를 사용하는 각 엔티티는
Annotation Context가 정의된 [엔티티 경로 계층](entity-path.md#path-hierarchy-functions)에서 가장 가까운 조상을 찾습니다.


## 세분화 이미지

세분화 이미지는 각 픽셀이 클래스 ID를 나타내는 단일 채널 정수 이미지/텐서입니다.
기본적으로 Rerun은 각 클래스 ID에 색상을 자동으로 할당하지만, Annotation Context를 정의함으로써
각 클래스의 색상을 명시적으로 결정할 수 있습니다.

* Python: [`rr.SegmentationImage`](https://ref.rerun.io/docs/python/stable/common/archetypes/#rerun.archetypes.SegmentationImage)
* Rust: [`rerun::SegmentationImage`](https://docs.rs/rerun/latest/rerun/archetypes/struct.SegmentationImage.html) 로깅

<picture>
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/segmentation_image/f48e7db9a1253f35b55205acd55d4b84ab1d8434/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/segmentation_image/f48e7db9a1253f35b55205acd55d4b84ab1d8434/768w.png">
  <img src="https://static.rerun.io/segmentation_image/f48e7db9a1253f35b55205acd55d4b84ab1d8434/full.png" alt="세분화 이미지의 스크린샷">
</picture>
