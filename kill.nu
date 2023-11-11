ps | filter {$in.name =~ "deno"} | each {kill $in.pid}
ps | filter {$in.name =~ "yt-music-download-ui"} | each {kill $in.pid}
