Самый простой механизм обработки ошибок, с которым мы познакомимся – это `panic`.
Он печатает сообщение с ошибкой, начинает процедуру
раскрутки стека и, чаще всего, завершает программу.

В данном примере мы явно вызывает `panic` в случае ошибки:

{panic.play}