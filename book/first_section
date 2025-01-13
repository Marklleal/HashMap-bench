# Performance Analysis of HashMap Accesses in Loops
## Why am I conducting this research?
While reading about <a href="https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value">"*updating a value based on the old value*"</a> in the **official rust book**, I came across a *for loop* that made me question why the author used a *variable* to get the *mutable value* from the **HashMap** *or_insert()* method instead of directly derefence it. 

```rust
# fn main() {
#   use std::collections::HashMap;
# 
#   let text = "hello world wonderful world";
#   let mut map: HashMap<&str, i32> = HashMap::new();
# 
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
#
#     println!("{map:?}");
# }
```

Couldn't we use the *get* method with an *if let* and have the same final result? Or something more reliable and with better clarity, like getting *frequency_map* value by index, removing the nesting of "if's". 
This intrigued me, so I decided to experiment by avoiding the inner variable *count*. I thought that since the count variable is created in every iteration of the for loop, avoiding it could reduce the time and improve the performance, right? 
Let's change the code above to be like what I've said so far:

```rust
# fn main() {
#   use std::collections::HashMap;
#   let text = "hello world wonderful world";
#   let mut map: HashMap<&str, i32> = HashMap::new();
# 
  for word in text.split_whitespace() {
    *map.entry(word).or_insert(0)+=1;
  }
#
#     println!("{map:?}");
# }
```

Now we just need to make some benchmarks and...

<div style="display: flex; gap: 10px; margin-bottom: 20px;">
  <div style="display: flex; gap: 10px; width: 100%;">
    <iframe id="iframe1" src="report_my_code/index.html" width="100%" height="500px" style="border: 1px solid black;"></iframe>
    <iframe id="iframe2" src="report_original_code/index.html" width="100%" height="500px" style="border: 1px solid black;"></iframe>
  </div>
</div>

It turns out the performance is basically the same in both cases (see the *Typical* additional **plot** or *slope estimate*), with this small difference being due to computer noise or variations in other processes running at the same time.
So, everthing was fine until I moved to the <a href="https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary">summary</a>, I've depared with a list of exercises to do, and the first exercise is the following:

> Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 
And how we'll do that? Guess what? Correctly, using one of the two code examples that we've covered so far. Ok, so... let's do some benchmarks for each code example to see what happens. But instead of calculating the median and the mode, we'll only focus on the mode. 
Since both approachs showed similar performance earlier, I don't expect much difference between the way I wrote my code and the way the book's author did.

```rust
#fn main() {
#    use std::collections::HashMap;
#    let numbers: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 12];
#    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
#    let mut max_frequency = 0;
#    let mut mode = numbers[0];
#
    for &num in numbers {
        let count = frequency_map.entry(num).or_insert(0);
        *count += 1;

        if *count > max_frequency {
            max_frequency += 1;
            mode = num
        }
    }
#    println!("{frequency_map:?} n {:?}", Some(mode));
#}
```

Above, maybe that's the way that the author of the book would imagine we would solve this exercise. But as I wanted to prove the efficiency of my code, I did it the following way:

```rust
#fn main() {
#    use std::collections::HashMap;
#    let numbers: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 12];
#    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
#    let mut max_frequency = 0;
#    let mut mode = numbers[0];
#
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;

        if frequency_map[&num] > max_frequency {
            max_frequency += 1;
            mode = num
        }
    }
#    println!("{frequency_map:?} n {:?}", Some(mode));
#}
```

Now, let see the difference in performance with criterion:

<div style="display: flex; gap: 10px;">
  <div style="display: flex; gap: 10px; width: 100%;">
    <iframe id="iframe3" src="report_my_exercise_code/index.html" width="100%" height="500px" style="border: 1px solid black;"></iframe>
    <iframe id="iframe4" src="report_original_exercise_code/index.html" width="100%" height="500px" style="border: 1px solid black;"></iframe>
  </div>
</div>

<script>
  document.addEventListener("DOMContentLoaded", () => {
  function syncIframes(iframeA, iframeB) {
    iframeA.addEventListener("load", () => {
      const winA = iframeA.contentWindow;
      const docA = iframeA.contentDocument || winA.document;

      iframeB.addEventListener("load", () => {
        const winB = iframeB.contentWindow;
        const docB = iframeB.contentDocument || winB.document;

        // Sincronizar scroll
        function syncScroll(source, target) {
          target.scrollTo(source.scrollX, source.scrollY);
        }

        winA.addEventListener("scroll", () => syncScroll(winA, winB));
        winB.addEventListener("scroll", () => syncScroll(winB, winA));

        // Sincronizar cliques
        function syncClick(event, targetWindow) {
          const clickEvent = new MouseEvent("click", {
            bubbles: true,
            cancelable: true,
            clientX: event.clientX,
            clientY: event.clientY,
            button: event.button,
          });
          targetWindow.document.elementFromPoint(event.clientX, event.clientY)?.dispatchEvent(clickEvent);
        }

        docA.addEventListener("click", (e) => syncClick(e, winB));
        docB.addEventListener("click", (e) => syncClick(e, winA));

        // Sincronizar atalhos de teclado (Alt + ←)
        function syncKeydown(event, targetWindow) {
          if (event.altKey && event.key === "ArrowLeft") {
            targetWindow.history.back();
          }
          if (event.altKey && event.key === "ArrowRight") {
            targetWindow.history.forward();
          }
        }

        docA.addEventListener("keydown", (e) => syncKeydown(e, winB));
        docB.addEventListener("keydown", (e) => syncKeydown(e, winA));
      });
    });
  }

  const iframe1 = document.getElementById("iframe1");
  const iframe2 = document.getElementById("iframe2");
  const iframe3 = document.getElementById("iframe3");
  const iframe4 = document.getElementById("iframe4");

  if (iframe1 && iframe2) syncIframes(iframe1, iframe2);
  if (iframe3 && iframe4) syncIframes(iframe3, iframe4);
});
</script>

If we analyze the typical plot in addtional plots, we see that the original code is about 20.53% faster than my code (estimate of 572.41 ns for my code and 455.13 ns for the original code). 
And... why? Before, the difference was less than 5% of difference (measure that is within the noise margin of the computer's processing). Well... the biggest problem here is the number of access to the HashMap! The original code makes **one** access per iteraction to the HashMap, meanwhile, my code makes **two** accesses. HashMaps aren't a simple data structure like arrays or something similar. 
Searching a HashMap requires hash calculations and comparisons to find the correct key, and this has a high cost to CPU. That's the reason for 20% difference between these two ways to solve the exercise.