# minigrep
rust官方文档中的minigrep项目

接收两个参数
参数1为待查找的字符串
参数2为目标文件

通过重定向 `>` 将文件中包含指定字符串的行输出到指定文件
错误信息会输出到标准错误打印

如下
`cargo run to poem.txt > output.txt`
表示在当前目录的poem.txt中查找包含字符串`to`的行，并将这些行写到output.txt中

可以通过环境变量`CASE_INSENSITIVE`来指定是否大小写敏感，当该环境变量被设置时，大小写不敏感，否则大小写敏感
在powershell中设置环境变量：
`$env:CASE_INSENSITIVE=1`
`cargo run to poem.txt > output.txt`
