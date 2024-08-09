---
title: Viewer를 상호작용으로 구성하기
order: 1
---

Rerun Viewer는 UI를 통해 직접 구성할 수 있습니다.

## Viewer 개요

<picture>
  <img src="https://static.rerun.io/overview/158a13691fe0364ed5d4dc420f5b2c39b60705cd/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/overview/158a13691fe0364ed5d4dc420f5b2c39b60705cd/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/overview/158a13691fe0364ed5d4dc420f5b2c39b60705cd/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/overview/158a13691fe0364ed5d4dc420f5b2c39b60705cd/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/overview/158a13691fe0364ed5d4dc420f5b2c39b60705cd/1200w.png">
</picture>

중앙 부분은 viewport라고 하며 데이터를 표시하는 다양한 뷰를 포함합니다.

Viewer의 왼쪽 패널은 "Blueprint 패널"입니다. 현재 blueprint의 내용을 나타내는 시각적 트리 뷰를 보여줍니다.

Viewer의 오른쪽 패널은 "Selection 패널"입니다. 이 패널에서는 현재 선택된 요소의 특정 blueprint 속성을 구성할 수 있습니다.

Blueprint는 viewport의 구조, 뷰 유형 및 내용을 정의합니다. Viewport의 내용을 변경하려면 blueprint를 편집해야 합니다.

Viewer를 편집한 후에는 [blueprint를 저장하거나 공유](./save-and-load.md)할 수 있습니다.

## 뷰 계층 구조 구성하기

Viewport는 다양한 뷰로 구성되며, 이들은 수직, 수평, 그리드 및 탭과 같은 다양한 종류의 중첩된 컨테이너로 계층적으로 배치됩니다. 이 계층 구조는 blueprint 패널에 표시되며, 최상위 컨테이너는 viewport에 해당합니다. 이 섹션에서는 이 뷰 계층 구조를 대화식으로 생성하고 수정하는 다양한 방법을 다룹니다.

### Blueprint의 일부 표시 또는 숨기기

"눈" 아이콘을 클릭하여 컨테이너나 뷰를 숨기거나 표시할 수 있습니다.

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/show_hide_btn/bbca385d4898ec220bfb91c430ea52d59553913e/full.png" alt="">
</picture>

### 새 컨테이너 또는 뷰 추가하기

Blueprint 패널 상단의 "+" 버튼을 클릭하여 viewport에 컨테이너나 뷰를 추가할 수 있습니다.

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/add_view/3933d7096846594304ddec2d51dda9c434d763bf/full.png" alt="">
</picture>

컨테이너(또는 viewport)가 이미 선택되어 있다면, selection 패널에서도 "+" 버튼을 사용할 수 있습니다.

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/add_view_selection_panel/2daf01c80dcd2496b554e4376af702c7713a47dc/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/add_view_selection_panel/2daf01c80dcd2496b554e4376af702c7713a47dc/480w.png">
</picture>

### 뷰 또는 컨테이너 제거하기

뷰나 컨테이너 옆의 "-" 버튼을 클릭하여 제거할 수 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/remove/6b9d97e4297738b8aad89158e4d15420be362b4a/full.png" alt="">
</picture>

### 기존 컨테이너 또는 뷰 재배치하기

Blueprint 패널에서 컨테이너나 뷰를 드래그 앤 드롭하여 viewport 계층 구조를 재구성할 수 있습니다. 또한 viewport에서 직접 뷰의 제목 탭을 사용하여 드래그할 수도 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/drag_and_drop_viewport/8521fda375a2f6af15628b04ead4ba848cb8bc27/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/drag_and_drop_viewport/8521fda375a2f6af15628b04ead4ba848cb8bc27/480w.png">
</picture>

### 뷰 또는 컨테이너 이름 변경하기

뷰와 컨테이너 모두 사용자 지정 이름을 지정할 수 있습니다. 뷰나 컨테이너를 선택한 후 selection 패널 상단에서 이름을 편집하면 됩니다.

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/rename/9dcb63d36f1676568fb106ee55ab110438b63fa9/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/rename/9dcb63d36f1676568fb106ee55ab110438b63fa9/480w.png">
</picture>

### 컨테이너 종류 변경하기

컨테이너는 수직, 수평, 그리드, 탭의 네 가지 종류가 있습니다. 기존 컨테이너의 종류를 변경하려면 해당 컨테이너를 선택하고 selection 패널의 드롭다운 메뉴에서 값을 변경하면 됩니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/container_kind/f123f2220d9e82d520af367b7af020179a4de675/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/container_kind/f123f2220d9e82d520af367b7af020179a4de675/480w.png">
</picture>

### 컨텍스트 메뉴 사용하기

Blueprint 패널에서 컨테이너나 뷰를 우클릭하여 컨텍스트 메뉴에 접근할 수 있습니다. 이전에 언급한 많은 작업들도 여기서 사용할 수 있습니다:

<picture>
  <img src="https://static.rerun.io/context_menu_container/e90e4688f306187d902467b452fb7146eec1bf4b/full.png" alt="">
</picture>

컨텍스트 메뉴를 사용하는 주요 장점은 여러 항목에 대해 한 번에 작업할 수 있다는 것입니다. 예를 들어, 여러 뷰를 선택하고(Ctrl-클릭 또는 Mac에서는 Cmd-클릭) 컨텍스트 메뉴를 사용하여 한 번에 모두 제거할 수 있습니다.

## 뷰 내용 구성하기

뷰의 내용은 entity query에 의해 결정되며, 이는 뷰가 선택되었을 때 selection 패널에서 수동으로 편집할 수 있습니다 (자세한 내용은 [Entity Queries](../../reference/entity-queries.md) 참조). 이 섹션에서는 뷰 내용을 조작하는 대화식 방법을 다룹니다 (이는 일반적으로 실제로 query를 수정하여 작동합니다).

### 뷰 내용 표시 또는 숨기기

컨테이너와 뷰와 마찬가지로, 뷰의 모든 entity는 "눈" 아이콘이나 컨텍스트 메뉴를 사용하여 표시하거나 숨길 수 있습니다.

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/show_hide_entity/587a5d8fd763c0bade461bc54a66a4acdd087821/full.png" alt="">
</picture>

### 뷰에서 데이터 제거하기

마찬가지로, entity 옆의 "-"를 클릭하여 뷰에서 제거할 수 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/remove_entity/ec0447ca7e420bc9d19a7bf015cc39f88b42598a/full.png" alt="">
</picture>

### Query 편집기 사용하기

뷰가 선택되었을 때 selection 패널에서 시각적 query 편집기를 사용할 수 있습니다. Entity query 옆의 "Edit" 버튼을 클릭하세요:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/add_remove_entity/4c5e536d4ca145058a8bc59a0b32267821663f06/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/add_remove_entity/4c5e536d4ca145058a8bc59a0b32267821663f06/480w.png">
</picture>

Query 편집기를 사용하면 query에서 entity와 entity 트리를 시각적으로 추가하고 제거할 수 있습니다.

### 컨텍스트 메뉴로 새 뷰에 entity 추가하기

Viewport 계층 구조와 마찬가지로, 뷰 데이터에 대한 대부분의 작업은 컨텍스트 메뉴에서 사용할 수 있습니다. 특히, blueprint 패널의 기존 뷰나 time 패널의 스트림에서 하나 이상의 entity를 선택하고 컨텍스트 메뉴에서 "Add to new space view"를 클릭하여 사용자 지정 내용으로 새 뷰를 만들 수 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/add_to_new_view/87f2d5ffb3ef896c82f398cd3c3d1c7321d59073/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/add_to_new_view/87f2d5ffb3ef896c82f398cd3c3d1c7321d59073/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/add_to_new_view/87f2d5ffb3ef896c82f398cd3c3d1c7321d59073/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/add_to_new_view/87f2d5ffb3ef896c82f398cd3c3d1c7321d59073/1024w.png">
</picture>

이 방법으로 권장 뷰 중 하나를 사용할 때, 뷰의 원점은 실제 데이터를 기반으로 자동으로 적절한 기본값으로 설정됩니다.

### Visualizer 및 컴포넌트 값 오버라이드하기

뷰에서 entity를 선택하면 어떤 visualizer를 활성화할지 지정하고 특정 컴포넌트의 값을 오버라이드할 수 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/visualizers/8ca9926398435e8b4c46eda91267a5454f6a75ba/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/visualizers/8ca9926398435e8b4c46eda91267a5454f6a75ba/480w.png">
</picture>

뷰를 선택할 때, 로그된 값이 없을 때 사용되는 주어진 유형의 컴포넌트에 대한 기본값을 지정할 수도 있습니다:

<picture style="zoom: 0.5">
  <img src="https://static.rerun.io/component_defaults/3330f16b246a523f9e9a9d8c3549cdc08660356f/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/component_defaults/3330f16b246a523f9e9a9d8c3549cdc08660356f/480w.png">
</picture>

See [Visualizers and Overrides](../../concepts/visualizers-and-overrides.md) for detailed information and more examples.

