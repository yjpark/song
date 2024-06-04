---
title: Example Guide
description: A guide in my new Starlight docs site.
---

## Sample Song

- Key: G
- Scale: Major
- Signature: 4/4
- Tempo: 72
- Tuning: Standard
- Capo: 4

### a
```chords
$duration = 1/2

| G Bm | C Cm |
```

### b.1
```chords
| G Bm | A/C# D |
```

### b.2
```chords
| G Bm | Am D |
```

### verse
```links
<a> <b.1> <a> <b.2>
```

### verse.melody
```melody
$duration = 1/8

32 | 3 5 - 63 5 - - 321 | 21 - 1 - .61 - - -
12 | 3 5 - 63 6 5 - 321 | 21 - 16 1 2 - -
```

### verse.lyrics
```lyrics
$duration = 1/8

热情 | 就 算 - 熄灭 了 - - 分手这 | 一晚 - 也 重要 - - -
甜言 | 蜜 语 - 谎话 嬉 笑 - 多给我 | 一点 - [x] 缺 少 - -
话题 | 尽 了 - 也不 紧 要 - 吻我至 | 凄冷 - 的 深 宵 - -
繁华 | 闹 市 - 灯光 普 照 - 然而 | 共你 _已 再 没破 - 晓 - - - |
```

### verse.1.lyrics
```lyrics
<verse.lyrics> {
    x = "不要"
}
```

### verse.2.lyrics
```links
<verse.lyrics> {
    x = "切勿"
}
```

### song
```links
<verse> <chorus> <verse> <chorus> 
```

