---
title: 코드를 통해 Viewer 구성하기
order: 3
---

이 튜토리얼은 Python에서 Rerun Viewer의 데이터 레이아웃과 외관을 더 잘 제어하기 위해
[Blueprint APIs](../../howto/configure-viewer-through-code.md)를 사용하는 방법을 안내합니다.

이 안내는 [주식 차트](https://github.com/rerun-io/rerun/tree/main/examples/python/blueprint_stocks) 예제를 기반으로 합니다.
이 튜토리얼과 링크된 예제의 주요 차이점은 명령줄 플래그의 추가 처리와 관련이 있으며,
여기서는 단순화를 위해 생략되었습니다.

이 튜토리얼의 모든 예제는 정확히 동일한 데이터를 사용합니다. 그러나 다음과 같은 작은 문장을 사용하여 blueprint를 변경함으로써:

```python
rrb.Blueprint(
    rrb.Vertical(
        rrb.TextDocumentView(name="Info", origin="/stocks/AAPL/info"),
        rrb.TimeSeriesView(name="Chart", origin="/stocks/AAPL"),
        row_shares=[1, 4],
    )
)
```
우리는 데이터가 표현되는 방식을 완전히 변경할 것입니다.

## 예제를 위한 환경 만들기

우리는 새로운 가상 환경을 만들고 Rerun SDK와 함께 이 예제에서 사용할 의존성들을 설치하는 것으로 시작합니다.

Linux나 Mac에서:

```bash
mkdir stocks_example
cd stocks_example
python -m venv venv
source venv/bin/activate
pip install rerun-sdk humanize yfinance
```

Windows에서:

```bash
mkdir stocks_example
cd stocks_example
python -m venv venv
.\venv\Scripts\activate
pip install rerun-sdk humanize yfinance
```

## 스크립트 생성하기

프로젝트 폴더에 새 파일 `stocks.py`를 추가합니다.

먼저, 필요한 라이브러리들을 import합니다:

```python
#!/usr/bin/env python3
import datetime as dt
import humanize
import pytz
import yfinance as yf
from typing import Any

import rerun as rr
import rerun.blueprint as rrb
```

다음으로, 스타일 데이터를 위한 헬퍼 함수와 정보 카드를 위한 템플릿을 생성합니다:

```python
brand_colors = {
    "AAPL": 0xA2AAADFF,
    "AMZN": 0xFF9900FF,
    "GOOGL": 0x34A853FF,
    "META": 0x0081FBFF,
    "MSFT": 0xF14F21FF,
}


def style_plot(symbol: str) -> rr.SeriesLine:
    return rr.SeriesLine(
        color=brand_colors[symbol],
        name=symbol,
    )


def style_peak(symbol: str) -> rr.SeriesPoint:
    return rr.SeriesPoint(
        color=0xFF0000FF,
        name=f"{symbol} (peak)",
        marker="Up",
    )


def info_card(
    shortName: str,
    industry: str,
    marketCap: int,
    totalRevenue: int,
    **args: dict[str, Any],
) -> rr.TextDocument:
    markdown = f"""
- **Name**: {shortName}
- **Industry**: {industry}
- **Market cap**: ${humanize.intword(marketCap)}
- **Total Revenue**: ${humanize.intword(totalRevenue)}
"""

    return rr.TextDocument(markdown, media_type=rr.MediaType.MARKDOWN)
```

마지막으로, 데이터를 쿼리하고 로깅하는 main 함수를 생성합니다:

```python
def main() -> None:
    symbols = ["AAPL", "AMZN", "GOOGL", "META", "MSFT"]

    # 시장 시간이 동부 시간대를 사용합니다
    et_timezone = pytz.timezone("America/New_York")
    start_date = dt.date(2024, 3, 18)
    dates = [start_date + dt.timedelta(days=i) for i in range(5)]

    # Rerun을 초기화하고 새로운 viewer를 생성합니다
    rr.init("rerun_example_blueprint_stocks", spawn=True)

    # 여기서 blueprint를 편집할 것입니다
    blueprint = None
    #rr.send_blueprint(blueprint)

    # 각 심볼과 날짜에 대한 주식 데이터를 로깅합니다
    for symbol in symbols:
        stock = yf.Ticker(symbol)

        # 주식 정보 문서를 timeless로 로깅합니다
        rr.log(f"stocks/{symbol}/info", info_card(**stock.info), timeless=True)

        for day in dates:
            # 스타일 데이터를 timeless로 로깅합니다
            rr.log(f"stocks/{symbol}/{day}", style_plot(symbol), timeless=True)
            rr.log(f"stocks/{symbol}/peaks/{day}", style_peak(symbol), timeless=True)

            # 시장 시간 동안 주식 데이터를 쿼리합니다
            open_time = dt.datetime.combine(day, dt.time(9, 30), et_timezone)
            close_time = dt.datetime.combine(day, dt.time(16, 00), et_timezone)

            hist = stock.history(start=open_time, end=close_time, interval="5m")

            # 인덱스를 시장 개장 이후의 초 단위로 오프셋합니다
            hist.index = hist.index - open_time
            peak = hist.High.idxmax()

            # 하루 동안의 주식 상태를 로깅합니다
            for row in hist.itertuples():
                rr.set_time_seconds("time", row.Index.total_seconds())
                rr.log(f"stocks/{symbol}/{day}", rr.Scalar(row.High))
                if row.Index == peak:
                    rr.log(f"stocks/{symbol}/peaks/{day}", rr.Scalar(row.High))


if __name__ == "__main__":
    main()

## 스크립트를 실행하세요

이제 스크립트를 실행하고 Rerun Viewer에서 결과를 볼 수 있습니다:

```bash
python stocks.py
```

애플리케이션이 시작되고 주식 데이터가 표시되는 것을 볼 수 있지만, 레이아웃이 이상적이지 않다는 것을 알 수 있습니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_no_blueprint/b7341f41683825f4186d661af509f8da03dc4ed1/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_no_blueprint/b7341f41683825f4186d661af509f8da03dc4ed1/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_no_blueprint/b7341f41683825f4186d661af509f8da03dc4ed1/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_no_blueprint/b7341f41683825f4186d661af509f8da03dc4ed1/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_no_blueprint/b7341f41683825f4186d661af509f8da03dc4ed1/1200w.png">
</picture>

## 블루프린트 생성

레이아웃을 개선하기 위해 이제 블루프린트 API를 사용하여 사용자 정의 레이아웃을 생성할 것입니다.

우리가 해야 할 일은 현재 읽고 있는 코드 섹션을 수정하는 것입니다:

```python
    # 여기서 블루프린트를 수정할 것입니다
    blueprint = None
    #rr.send_blueprint(blueprint)
```

### 원본에 대한 뷰 생성

다음 줄을 다음으로 교체하세요:

```python
    # Create a single chart for all the AAPL data:
    blueprint = rrb.Blueprint(
        rrb.TimeSeriesView(name="AAPL", origin="/stocks/AAPL"),
    )
    rr.send_blueprint(blueprint)
```

이 블루프린트는 `origin` 매개변수를 사용하여 엔티티 트리의 특정 부분만 뷰에 포함시킵니다.

스크립트를 다시 실행하면 AAPL 데이터에 대한 단일 차트를 볼 수 있습니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_one_stock/bda8f536306f9d9eb1b2aafe8bd8aceb746c2e0c/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock/bda8f536306f9d9eb1b2aafe8bd8aceb746c2e0c/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock/bda8f536306f9d9eb1b2aafe8bd8aceb746c2e0c/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock/bda8f536306f9d9eb1b2aafe8bd8aceb746c2e0c/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock/bda8f536306f9d9eb1b2aafe8bd8aceb746c2e0c/1200w.png">
</picture>

### 기본 패널 상태 제어

데이터를 제어하는 것 외에도 blueprint, selection, 그리고 time 패널의 기본 상태를 제어할 수 있습니다.

이러한 추가적인 blueprint 사양을 포함하도록 코드를 다시 수정해 보겠습니다:

```python
    # AAPL 데이터에 대한 단일 차트를 생성하고, 선택 및 시간 패널을 접습니다:
    blueprint = rrb.Blueprint(
        rrb.TimeSeriesView(name="AAPL", origin="/stocks/AAPL"),
        rrb.BlueprintPanel(state="expanded"),
        rrb.SelectionPanel(state="collapsed"),
        rrb.TimePanel(state="collapsed"),
    )
    rr.send_blueprint(blueprint)
```

이번에 스크립트를 실행하면 패널들이 처음에 접혀있는 상태로 시작되어, 데이터에 대해 더 집중된 뷰를 제공합니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_one_stock_hide_panels/41d3f42d2e33bcaec33b27e98752eddb17352c0f/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_hide_panels/41d3f42d2e33bcaec33b27e98752eddb17352c0f/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_hide_panels/41d3f42d2e33bcaec33b27e98752eddb17352c0f/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_hide_panels/41d3f42d2e33bcaec33b27e98752eddb17352c0f/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_hide_panels/41d3f42d2e33bcaec33b27e98752eddb17352c0f/1200w.png">
</picture>

### 여러 뷰 결합하기

blueprint를 사용할 때, 단일 뷰로 제한할 필요가 없습니다. 여러 뷰를 생성하고
컨테이너를 사용하여 이들을 결합할 수 있습니다.

info 카드도 포함하도록 코드를 수정해 보겠습니다. `Vertical` 컨테이너와
`row_shares` 매개변수를 사용하여 뷰의 상대적 크기를 제어할 것입니다:

```python
    # info 문서와 시계열 차트의 수직 레이아웃 생성
    blueprint = rrb.Blueprint(
        rrb.Vertical(
            rrb.TextDocumentView(name="Info", origin="/stocks/AAPL/info"),
            rrb.TimeSeriesView(name="Chart", origin="/stocks/AAPL"),
            row_shares=[1, 4],
        ),
        rrb.BlueprintPanel(state="expanded"),
        rrb.SelectionPanel(state="collapsed"),
        rrb.TimePanel(state="collapsed"),
    )
    rr.send_blueprint(blueprint)
```

이제 스크립트를 실행하면 두 개의 뷰가 수직으로 쌓인 형태로 나타납니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_one_stock_and_info/9fbf481aaf9da399718d8afb9f64b9364bb34268/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_and_info/9fbf481aaf9da399718d8afb9f64b9364bb34268/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_and_info/9fbf481aaf9da399718d8afb9f64b9364bb34268/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_and_info/9fbf481aaf9da399718d8afb9f64b9364bb34268/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_and_info/9fbf481aaf9da399718d8afb9f64b9364bb34268/1200w.png">
</picture>

### 특정 내용 포함하기

뷰의 `origin`을 지정하는 것은 편리하지만, 때로는 더 많은 제어가 필요합니다. 이 경우, 여러 개의 content 표현식을 제공하여 뷰의 `contents`를 지정할 수 있습니다.

예를 들어, 단일 차트에 META와 MSFT 두 회사의 하루 데이터를 포함하는 주식 차트를 만들 수 있습니다. `origin`만 사용해서는 이를 표현할 방법이 없습니다:

```python
    # 두 개의 주식 시계열을 가진 뷰 생성
    blueprint = rrb.Blueprint(
        rrb.TimeSeriesView(
            name="META vs MSFT",
            contents=[
                "+ /stocks/META/2024-03-19",
                "+ /stocks/MSFT/2024-03-19",
            ],
        ),
        rrb.BlueprintPanel(state="expanded"),
        rrb.SelectionPanel(state="collapsed"),
        rrb.TimePanel(state="collapsed"),
    )
    rr.send_blueprint(blueprint)
```

이제 스크립트를 실행하면 여러 소스의 데이터를 결합한 차트가 생성됩니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_comare_two/0ac7d7d02bebb433828aec16a085716951740dff/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_comare_two/0ac7d7d02bebb433828aec16a085716951740dff/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_comare_two/0ac7d7d02bebb433828aec16a085716951740dff/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_comare_two/0ac7d7d02bebb433828aec16a085716951740dff/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_comare_two/0ac7d7d02bebb433828aec16a085716951740dff/1200w.png">
</picture>

### 더 복잡한 필터링

단일 경로 포함만 지정하는 것은 대규모 하위 트리를 포함하는 데이터셋을 다룰 때 어려울 수 있습니다.

필터 표현식을 사용하여 경로 패턴을 기반으로 데이터를 포함하거나 제외할 수 있습니다. 이 패턴은 선택적으로
`$origin`으로 시작하여 주어진 공간의 원점을 참조할 수 있으며, 와일드카드 `/**`로 끝나 전체 하위 트리를
포함하거나 제외할 수 있습니다.

단일 주식 예제로 돌아가서, `peaks` 하위 트리를 제외함으로써 peaks 데이터를 필터링할 수 있습니다:

```python
    # AAPL의 모든 데이터에 대한 단일 차트를 생성하고 peaks를 필터링합니다:
    blueprint = rrb.Blueprint(
        rrb.TimeSeriesView(
            name="AAPL",
            origin="/stocks/AAPL",
            contents=[
                "+ $origin/**",
                "- $origin/peaks/**",
            ],
        ),
        rrb.BlueprintPanel(state="expanded"),
        rrb.SelectionPanel(state="collapsed"),
        rrb.TimePanel(state="collapsed"),
    )
    rr.send_blueprint(blueprint)
```

스크립트를 실행하면 peaks 하위 트리의 데이터가 더 이상 뷰에 포함되지 않은 것을 볼 수 있습니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_one_stock_no_peaks/d53c5294e3ee118c5037d1b3480176ef49cb2071/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_no_peaks/d53c5294e3ee118c5037d1b3480176ef49cb2071/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_no_peaks/d53c5294e3ee118c5037d1b3480176ef49cb2071/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_no_peaks/d53c5294e3ee118c5037d1b3480176ef49cb2071/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_one_stock_no_peaks/d53c5294e3ee118c5037d1b3480176ef49cb2071/1200w.png">
</picture>

### 프로그래매틱 레이아웃

이러한 레이아웃은 Python 코드를 실행하여 생성되므로 프로그래매틱하게 생성할 수도 있습니다.

예를 들어, 우리가 관심 있는 모든 데이터에 대해 별도의 뷰를 생성할 수 있습니다.
이를 수동으로 설정하는 것은 매우 지루할 것입니다.

```python
    # 모든 심볼과 날짜를 반복하여 주식 데이터를 그리드에 기록합니다
    blueprint = rrb.Blueprint(
        rrb.Vertical(
            contents=[
                rrb.Horizontal(
                    contents=[
                        rrb.TextDocumentView(
                            name=f"{symbol}",
                            origin=f"/stocks/{symbol}/info",
                        ),
                    ]
                    + [
                        rrb.TimeSeriesView(
                            name=f"{day}",
                            origin=f"/stocks/{symbol}/{day}",
                        )
                        for day in dates
                    ],
                    name=symbol,
                )
                for symbol in symbols
            ]
        ),
        rrb.BlueprintPanel(state="expanded"),
        rrb.SelectionPanel(state="collapsed"),
        rrb.TimePanel(state="collapsed"),
    )
    rr.send_blueprint(blueprint)
```

스크립트를 다시 실행하면 이 최종 차트가 원래의 휴리스틱 기반 레이아웃에 비해 크게 개선된 것을 볼 수 있습니다:

<picture>
  <img src="https://static.rerun.io/blueprint_tutorial_grid/b9c41481818f9028d75df6076c62653989a02c66/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/blueprint_tutorial_grid/b9c41481818f9028d75df6076c62653989a02c66/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/blueprint_tutorial_grid/b9c41481818f9028d75df6076c62653989a02c66/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/blueprint_tutorial_grid/b9c41481818f9028d75df6076c62653989a02c66/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/blueprint_tutorial_grid/b9c41481818f9028d75df6076c62653989a02c66/1200w.png">
</picture>

### 시각화 도구 및 오버라이드

<!-- TODO(ab): 연결된 섹션의 내용이 이미 꽤 풍부하지만, 이상적으로는 이 섹션에도 코드 예제가 포함되어야 합니다 -->

0.17 릴리스 이후로 코드를 통한 더 깊은 구성이 가능해졌습니다. 여기에는 주어진 뷰 엔티티에 대한 컴포넌트 값 오버라이드, 주어진 뷰에 대한 컴포넌트의 기본값 지정, 그리고 뷰 엔티티별로 사용되는 시각화 도구를 제어하는 것이 포함됩니다. 자세한 정보와 코드 예제는 [시각화 도구 및 오버라이드](../../concepts/visualizers-and-overrides.md)를 참조하세요.
