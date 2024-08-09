---
title: Blueprint
order: 600
---

## Blueprint과 녹화

<!-- source: Rerun Design System/Documentation schematics -->
<img src="https://static.rerun.io/6e2095a0ffa4f093deb59848b7c294581ded4678_blueprints_and_recordings.png" width="550px">

Rerun 뷰어로 작업할 때, 시각적 요소를 생성하는 두 가지 요소가 있습니다: "녹화"와 "blueprint"입니다.

-   녹화는 시각화하려는 실제 데이터를 제공합니다.
-   Blueprint는 녹화의 데이터가 어떻게 표시될지 결정하는 구성입니다.

이 두 요소 모두 중요합니다 — 녹화 없이는 보여줄 데이터가 없고, blueprint 없이는 보여줄 방법이 없습니다. 이전에 blueprint를 명시적으로 로드하지 않고 Rerun을 사용했더라도, 실제로 Viewer가 당신을 위해 blueprint를 생성하고 있었습니다. Blueprint 없이는 Viewer가 표시할 수 있는 것이 문자 그대로 아무것도 없습니다.

## 느슨한 결합

Blueprint와 녹화는 느슨하게 결합되어 있습니다. Rerun은 [애플리케이션 ID](apps-and-recordings.md)를 사용하여 blueprint와 녹화가 함께 사용되어야 하는지 결정하지만, 그 이상으로 직접적으로 연결되어 있지는 않습니다.

이는 둘 중 하나를 독립적으로 변경할 수 있음을 의미합니다. Blueprint를 일정하게 유지하면서 녹화를 변경하면 일관된 뷰 세트를 사용하여 다른 데이터셋을 비교할 수 있습니다. 반대로, 녹화를 일정하게 유지하면서 blueprint를 변경하면 동일한 데이터를 다른 방식으로 볼 수 있습니다.

## Blueprint가 제어하는 것

Viewer가 표시하는 모든 측면은 blueprint에 의해 제어됩니다. 여기에는 다양한 뷰의 유형과 내용, 다양한 컨테이너의 구성과 레이아웃, 그리고 개별 데이터 시각화 도구의 구성 및 스타일 속성이 포함됩니다 (자세한 내용은 [시각화 도구 및 오버라이드](visualizers-and-overrides.md)를 참조하세요).

일반적으로, viewer를 통해 무언가의 모습을 수정할 수 있다면 실제로 blueprint를 수정하고 있는 것입니다. (현재 이 규칙에 몇 가지 예외가 있을 수 있지만, 궁극적으로는 모든 상태를 blueprint로 마이그레이션하는 것이 목표입니다.)

## 현재, 기본, 그리고 휴리스틱 blueprint

<!-- source: Rerun Design System/Documentation schematics -->
<img src="https://static.rerun.io/fe1fcf086752f5d7cdd64b195fb3a6cb99c50737_current_default_heuristic.png" width="550px">

Blueprint는 여러 소스에서 생성될 수 있습니다.

- 주어진 애플리케이션 ID에 대한 "현재 blueprint"는 Viewer가 데이터를 표시하는 데 사용하는 것입니다. 이는 viewer 내에서 시각화에 대한 각 변경사항에 대해 업데이트되며, 언제든지 blueprint 파일로 저장할 수 있습니다.
- "기본 blueprint"는 코드에서 받거나 파일에서 로드된 blueprint가 설정되거나 업데이트될 때의 스냅샷입니다. 현재 blueprint는 blueprint 패널 헤더의 "reset" 버튼을 사용하여 언제든지 기본 blueprint로 재설정할 수 있습니다.
- "휴리스틱 blueprint"는 녹화 데이터를 기반으로 자동으로 생성된 blueprint입니다. 기본 blueprint를 사용할 수 없을 때, 현재 blueprint를 재설정할 때 휴리스틱 blueprint가 사용됩니다. 또한 애플리케이션을 선택한 후 선택 패널에서 휴리스틱 blueprint로 재설정할 수 있습니다.

## Blueprint란 무엇인가

내부적으로, blueprint는 단순히 데이터입니다. 이는 녹화와 마찬가지로 [시계열 ECS](./entity-component.md)로 표현됩니다. 유일한 차이점은 특정 blueprint 아키타입 세트와 특별한 blueprint 타임라인을 사용한다는 것입니다. blueprint가 동일한 연결을 통해 전송될 수 있지만, blueprint 데이터는 격리된 저장소에 보관되며 녹화 데이터와 혼합되지 않습니다.

blueprint 작업을 위한 Rerun API가 일반 로깅 API와 다르게 보일 수 있지만, 실제로는 blueprint 특정 아키타입 모음을 별도의 blueprint 스트림에 로깅하기 위한 구문적 설탕일 뿐입니다.

또한, UI에서 Viewer를 변경할 때 실제로 일어나는 일은 Viewer가 새로운 blueprint 이벤트를 생성하고 이를 blueprint 저장소의 blueprint 타임라인 끝에 추가하는 것입니다.

## Viewer 작동

주로 성능을 위한 캐싱 외에는 viewer는 프레임 간에 매우 적은 상태를 유지합니다. 목표는 Viewer의 출력이 blueprint와 녹화의 결정론적 함수가 되는 것입니다.

매 프레임마다 Viewer는 "활성" blueprint와 "활성" 녹화의 최소한의 컨텍스트로 시작합니다. 그런 다음 Viewer는 blueprint 타임라인의 현재 리비전을 사용하여 blueprint 저장소에서 컨테이너와 space-view 아키타입을 쿼리합니다. space-view 아키타입은 차례로 뷰를 렌더링하기 위해 녹화 저장소에서 쿼리해야 하는 경로 유형을 지정합니다.

blueprint를 수정하는 모든 사용자 상호작용은 대기열에 추가되고 blueprint 타임라인의 다음 리비전을 사용하여 blueprint에 다시 기록됩니다.

## Blueprint 아키텍처 동기

이 아키텍처가 약간의 복잡성과 간접성을 추가하지만, Viewer가 모든 의미 있는 프레임 간 상태를 구조화된 blueprint 데이터 저장소에 저장한다는 사실은 여러 가지 장점이 있습니다:

-   Viewer에서 수정하는 모든 것을 blueprint로 저장하고 공유할 수 있습니다.
-   Viewer 라이브러리에 의존하지 않고 Rerun SDK만 사용하여 프로그래밍 방식으로 blueprint를 생성할 수 있습니다.
-   Blueprint는 녹화가 표현할 수 있는 모든 데이터를 표현할 수 있습니다. 이는 blueprint 소스 데이터 [오버라이드](visualizers-and-overrides.md#per-entity-component-override)가 로깅된 데이터만큼 표현력이 있다는 것을 의미합니다.
-   Blueprint는 실제로 전체 시계열로 저장되어 스냅샷 및 실행 취소/다시 실행 메커니즘과 같은 기능의 향후 구현을 단순화합니다.
-   일반 Rerun 데이터를 검사하기 위한 디버깅 도구를 사용하여 내부 blueprint 상태를 검사할 수 있습니다.
