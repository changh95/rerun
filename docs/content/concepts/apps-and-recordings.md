---
title: 애플리케이션 ID 및 녹화 ID
order: 500
---

## Application ID
Rerun을 [`rr.init`](https://ref.rerun.io/docs/python/stable/common/initialization_functions/#rerun.init)으로 초기화할 때 Application ID를 설정해야 합니다.

당신의 Rerun Viewer는 이 애플리케이션 ID를 기반으로 Blueprint를 저장합니다.
이는 당신이 앱을 실행하고 Viewer를 원하는 대로 설정할 수 있음을 의미하며,
다시 앱을 실행할 때 Rerun Viewer는 당신이 Space Views 등을 설정한 방식을 기억합니다.

## Recording ID
Rerun을 사용하여 로깅을 시작할 때마다 무작위 _Recording ID_가 생성됩니다.
예를 들어, 각 `.rrd` 파일은 고유한 Recording ID를 가집니다.

이는 서로 다른 Recording ID를 가진 여러 녹화를 동일한 애플리케이션 ID와 공유할 수 있음을 의미합니다.

여러 프로세스에서 로깅하고 모든 로그 데이터가 Viewer에 함께 표시되기를 원한다면,
모든 프로세스가 동일한 Recording ID를 사용하도록 해야 합니다.
