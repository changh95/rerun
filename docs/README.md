Rerun의 상위 문서는 https://www.rerun.io/docs에서 호스팅됩니다.

## 기타 문서
API 수준의 문서는 소스 코드에서 빌드됩니다. Python은 [rerun_py](https://github.com/rerun-io/rerun/tree/main/rerun_py)에 있고 Rust는 개별 [crates](https://github.com/rerun-io/rerun/tree/main/crates)에 있습니다.

## 기여

기여는 pull-request를 통해 환영합니다. 착륙한 PR도 다음 사이트 업데이트를 롤아웃할 때까지 메인 사이트에 배포되지 않습니다. 일반적으로 릴리스당 최소 한 번은 이를 수행합니다.

## 조직

사이트 문서는 [`/content`](./content) 내부의 Markdown 파일에 있습니다.

문서의 진입점은 [`/content/index.md`](./content/index.md)입니다.

## 문서 업데이트

`rerun.io` 문서는 `docs-latest` 브랜치의 `/docs` 디렉토리 내용에서 빌드됩니다. `docs-latest`에 대한 모든 푸시는 웹사이트의 자동 재배포를 트리거합니다.

`docs-latest` 브랜치에 직접 푸시하지 마십시오! 문서를 업데이트하려면 [Release 생성](../RELEASES.md)을 하거나 커밋이 `main`에 커밋된 후 `docs-latest` 브랜치에 체리픽하십시오.

⚠ `main`에 없는 커밋은 `docs-latest` 브랜치에 직접 제출된 경우 릴리스 프로세스 중에 `docs-latest` 브랜치가 강제로 푸시되기 때문에 다음 릴리스를 생성할 때 손실됩니다.

## 특수 구문

### Frontmatter

YAML frontmatter at the top of the Markdown file is used for metadata:

```
---
title: Examples
order: 6
---
```

The available attributes are:
| name       | type    | required | description                                         |
| ---------- | ------- | -------- | --------------------------------------------------- |
| title      | string  | yes      | navigation item title                               |
| order      | number  | yes      | used to sort navigation items                       |
| redirect   | string  | no       | redirect to the given url                           |
| hidden     | boolean | no       | don't show the item in navigation                   |
| expand     | boolean | no       | expand the sub-items in navigation by default       |
| ogImageUrl | string  | no       | url to an image to show as the open-graph thumbnail |

### Snippets

Snippets are small, self-contained code examples.

To ensure all the code blocks in our documentation contain valid code, we give each one a name, and move it into `snippets/all`:
```
/docs
  /snippets
    /all
      /my-example
        /example.py
```

Each snippet can and should be written in all our supported languages:
```
/docs
  /snippets
    /all
      /my-example
        /example.cpp
        /example.py
        /example.rs
```

Once the snippet is in `snippet/all`, it may be referenced in a documentation Markdown file using this syntax:
```
snippet: my-example
```

### Screenshot links

If a screenshot shows an example or snippet which is runnable and built on CI, then you can turn the screenshot
to a link to `rerun.io/viewer` pointing at the example using the `data-inline-viewer` attribute.

Add the attribute to any `<picture>` element like so:

```html
<picture data-inline-viewer="snippets/segmentation_image_simple">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/1200w.png">
  <img src="https://static.rerun.io/segmentation_image_simple/eb49e0b8cb870c75a69e2a47a2d202e5353115f6/full.png">
</picture>
```

The value should be:
- `examples/{NAME}` for examples
- `snippets/{NAME}` for snippets
