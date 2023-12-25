# Advent of Code 2023

## Quick setup
``` shell
rustup default stable
cargo install cargo-generate
brew install just
```

## Some commands
Create package for a new day:
```shell
just create <day>
```

Execute binary for a given day and task:
```shell
just run <day> <part>
```
where `part` is either `part01` or `part02`

## Calendar
<pre><span title="Day 14, two stars" >                     <span>...'''''''''...</span>                    
                  <span>.''</span> <span>~</span><span>/\</span><b>*</b> <span>~~~~</span>  <span>/\</span> <span>''.</span>            <span>14</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/14" target="_blank">puzzle</a> | <a href="day-14/src/lib.rs">solution</a></span>
<span title="Day 15, two stars" >                <span>.'</span> <span>/\/\</span> <span>~~~~~~~~</span> <b>*</b> <span>/</span><span>~</span><span>\</span> <span>'.</span>          <span>15</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/15" target="_blank">puzzle</a> | <a href="day-15/src/lib.rs">solution</a></span>
<span title="Day 16, two stars" >           <span>...</span>  <span>:</span><span>~~~~~~~~~</span> <span>/\</span> <span>~~~~</span><span>/</span> <b>*</b> <span>\</span> <span>:</span>          <span>16</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/16" target="_blank">puzzle</a> | <a href="day-16/src/lib.rs">solution</a></span>
<span title="Day 13, two stars" >    <span>.''....'</span> <span>'..</span><span>'.<span style=""></span></span><span>~~~~</span><span>/\</span><b>*</b><span>/\</span> <span>~</span>  <span>/\</span>   <span>/\</span> <span>.'</span>          <span>13</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/13" target="_blank">puzzle</a> | <a href="day-13/src/lib.rs">solution</a></span>
<span title="Day 17, two stars" >    <span>'.ZZ</span><span>~</span>   <span>~</span> <b>*</b> <span>~~~~</span><span>.</span>   <span>/\</span>  <span>/\</span>   <span>/\</span> <span>..'</span>            <span>17</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/17" target="_blank">puzzle</a> | <a href="day-17/src/lib.rs">solution</a></span>
<span title="Day 12, two stars" ><span>.''''</span> <span>ZZ</span><b>*</b> <span>.'''.[]</span><span>~~~</span><span>'</span><span>'''.........'''</span>               <span>12</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/12" target="_blank">puzzle</a> | <a href="day-12/src/lib.rs">solution</a></span>
<span title="Day 18, two stars" ><span>'....</span> <span>~</span>   <span>'...'</span>  <b>*</b><span>[]....'</span>                          <span>18</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/18" target="_blank">puzzle</a> | <a href="day-18/src/lib.rs">solution</a></span>
<span title="Day 11, two stars" >    <span>.'</span><b>*</b>  <span>~</span>   <span>[^^^]</span> <span>'.</span>                              <span>11</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/11" target="_blank">puzzle</a> | <a href="day-11/src/lib.rs">solution</a></span>
<span title="Day 10, two stars" >    <span>'..''''.</span><b>*</b><span>.''''.</span>.'</span><span>'</span> <span>''...</span>                       <span>10</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/10" target="_blank">puzzle</a> | <a href="day-10/src/lib.rs">solution</a></span>
<span title="Day 19, two stars" >          <span>.</span><span>'''</span><span>~</span> <span>~</span> <span>~</span> <span>.</span> <b>*</b> <span>###</span> <span>''.</span>                    <span>19</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/19" target="_blank">puzzle</a> | <a href="day-19/src/lib.rs">solution</a></span>
<span title="Day 9, two stars" >        <span>.'</span> <span>~</span>  <span>,</span><b>*</b> <span>~</span> <span>'",</span> <span>~</span> <span>#####</span> <span>'.</span>                  <span> 9</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/9" target="_blank">puzzle</a> | <a href="day-09/src/lib.rs">solution</a></span>
<span title="Day 8, two stars" >        <span>:</span> <span>~</span> <span>'</span><span>(~)</span><span>,</span> <span>~</span> <b>*</b> <span>~</span> <span>~</span> <span>~</span> <span>###</span> <span>:</span>                  <span> 8</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/8" target="_blank">puzzle</a> | <a href="day-08/src/lib.rs">solution</a></span>
<span title="Day 20, two stars" >        <span>'.</span> <span>~</span> <span>"</span> <span>'</span> <span>~</span> <span>~</span> <span>~</span> <b>*</b> <span>~~~</span><span>##</span> <span>.'</span>                  <span>20</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/20" target="_blank">puzzle</a> | <a href="day-20/src/lib.rs">solution</a></span>
<span title="Day 7, two stars" >          <span>'..</span> <span>~</span> <span>~</span> <b>*</b> <span>~</span> <span>####</span><span>~~~~'</span><span>.'''''''''...</span>       <span> 7</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/7" target="_blank">puzzle</a> | <a href="day-07/src/lib.rs">solution</a></span>
<span title="Day 6, two stars" >             <span>'''.........'''</span><span>'</span> <span>~</span> <span>.'</span><b>*</b><span>.</span> <span>~</span>  <span>..</span>  <span>''.</span>    <span> 6</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/6" target="_blank">puzzle</a> | <a href="day-06/src/lib.rs">solution</a></span>
<span title="Day 21, two stars" >                        <span>.'</span> <span>~</span> <span>..</span> <span>'...'</span> <span>~</span><span>'</span><b>*</b> <span>'.</span><span>~</span>  <span>'.</span>  <span>21</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/21" target="_blank">puzzle</a> | <a href="day-21/src/lib.rs">solution</a></span>
<span title="Day 5, two stars" >                        <span>~~</span> <span>.~~~'.</span> <span>~</span>     <span>'.</span> <b>*</b><span>'.</span><span>~</span> <span>:</span>  <span> 5</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/5" target="_blank">puzzle</a> | <a href="day-05/src/lib.rs">solution</a></span>
<span title="Day 22, two stars" >                 <span>...''''</span><span>~~~</span><span>'</span><b>*</b><span>~~.'</span>  <span>.''.</span><span>~</span>  <span>'..'</span> <span>.'</span>  <span>22</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/22" target="_blank">puzzle</a> | <a href="day-22/src/lib.rs">solution</a></span>
<span title="Day 4, two stars" >              <span>.''</span>   <span>-</span>   <span>-</span> <span>'..</span>  <span>~</span><span>..'</span><b>*</b>   <span>'.</span> <span>~</span> <span>..'</span>    <span> 4</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/4" target="_blank">puzzle</a> | <a href="day-04/src/lib.rs">solution</a></span>
<span title="Day 23, two stars" >            <span>.'</span> <b>*</b> <span>-</span>    <span>/\</span> <span>-</span>   <span>'''..</span><span>/</span><span>......'''</span>       <span>23</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/23" target="_blank">puzzle</a> | <a href="day-23/src/lib.rs">solution</a></span>
<span title="Day 25, two stars" >            <span>:</span>  <span>-</span>   <span>-</span>  <span>-</span> <b>*</b> <span>/\</span>    <span>-</span><span>/</span>  <span>:</span>              <span>25</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/25" target="_blank">puzzle</a> | <a href="day-25/src/lib.rs">solution</a></span>
<span title="Day 24, two stars" >            <span>'.</span>    <span>-</span> <b>*</b>  <span>/\</span> <span>-</span>   <span>-</span> <span>/</span>  <span>.'</span>              <span>24</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/24" target="_blank">puzzle</a> | <a href="day-24/src/lib.rs">solution</a></span>
<span title="Day 3, two stars" >              <span>'..</span>    <span>-</span>     <span>-</span>   <b>*</b><span>..'</span>                <span> 3</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/3" target="_blank">puzzle</a> | <a href="day-03/src/lib.rs">solution</a></span>
<span title="Day 2, two stars" >    <span>----@</span>        <span>'''..</span><b>*</b><span>......'''</span>                   <span> 2</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/2" target="_blank">puzzle</a> | <a href="day-02/src/lib.rs">solution</a></span>
<span title="Day 1, two stars" >  <b>*</b> <span>!</span> <span>/^\</span>                                          <span> 1</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/1" target="_blank">puzzle</a> | <a href="day-01/src/lib.rs">solution</a></span>
</pre>