## 响应式数据

#### 数据源
data和props是每个元素的数据源。数据源可以用于渲染、缓存和传递给子元素。
|                | data             | props |
| -------------- | ---------------- | ----- |
| 用户可赋值     | 是（当可访问时） | 是    |
| 元素可赋值     | 是               | 否    |
| 修改将触发重绘 | 是               | 是    |
为保证数据单向流动，我们不允许元素对props进行赋值。

#### 数据结构

我们来简化概述中的那张结构图，省略本章节不关心的部分：
```
元素<'a> {
    props1: &'a String,
    props2: AnotherThing<'a>,

    __core(自动解引用): 储存单元<'static>{
        data1: String,
        data2: u32,
        ...
    },

    ...
}
```

首先，一个元素必须通过例如`elem.field()`或`elem.field_mut()`的方法[^call_data]来访问需要的数据，图中的`props1`、`data1`等在实际情况下都是随机名字。因为在编译期，所有元素之间的依赖关系会确定下来，而这之间的依赖关系就通过方法来实现。

[^call_data]:因为方法调用的缘故，如果同时存在`field`和`field_mut`两个域，宏将会尝试生成`field`、`field_mut`和`field_mut`、`field_mut_mut`方法，显然两个`field_mut`存在冲突，并在编译期报错。为避免这种情况发生，建议不要以`_mut`作为域的结尾，但是并不强制要求。