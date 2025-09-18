/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// I AM NOT DONE
#[derive(Debug)]
struct Stack<T> {   // 栈结构体
	size: usize,	// 栈长度
	data: Vec<T>,	// 使用Vec存储栈元素
}
impl<T> Stack<T> {
	fn new() -> Self {	// 新建栈
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool { // 判空
		0 == self.size
	}
	fn len(&self) -> usize { 	// 返回长度
		self.size
	}
	fn clear(&mut self) {	// 清除栈
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {	// 压栈
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> { // 出栈
		// TODO
		if 0 == self.size {
			return None;
		} else {
			self.size -= 1;
			self.data.pop()  // Vec的pop方法弹出并返回尾部元素
		}
	}
	fn peek(&self) -> Option<&T> { // 获取不可变栈顶元素
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {	// 获取可变栈顶元素
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {	// 将栈转换为迭代器
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {	// 迭代器
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {	// 返回可变迭代器
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	let mut st = Stack::new();

	for c in bracket.chars() {
		if ['(', '[', '{'].contains(&c) {
			st.push(c);
		} else {
			if st.is_empty() {
				return false;
			}
			let top_c = st.pop().unwrap();		// 	弹出栈顶的括号
			if (c == ')' && top_c != '(') ||	// 检查左右括号是否匹配
				(c == ']' && top_c != '[') ||
				(c == '}' && top_c != '{') {
				return false;
			}
		}
	}

	st.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}