# completion

Using completion variables is an easy way to synchronize between two tasks in
the kernel when one task needs to signal to the other that an event has
occurred. One task waits on the completion variable while another task performs
some work. When the other task has completed the work, it uses the completion
variable to wake up any waiting tasks.

## build the module

To build this module, execute:

```bash
make KERNELDIR=/path/to/kernel/source/dir
```

If you have already set and exported `KERNELDIR` environment variable, simply
execute `make` is enough.

If neither `KERNELDIR` environment variable nor `KERNELDIR` option of make
are set, the current running kernel will be built against.

## Usage

Copy **completion.ko** and **load_module.sh** file to the target machine,
then run:

```bash
./load_module.sh
```

## test the module

In this example, we need two consoles in which we can perform read and write
separately. You can use any remote login tools, such as ssh or telnet, to open
another console. For users who followed my QEMU dev setup instruction, it
should already successfully have a telnetd daemon running. For more details,
check homework_telnet_deployment more details.

After the kernel module is loaded successfully. Execute the following command
in console 1:

```bash
cat /dev/completion
```

This command will block the current process, waiting for that if some other
process write something into the same file.

Here, in console 2, execute `echo "something" > /dev/completion`. After this,
the **cat** process previously executed in console 1 is waken up.

---

## experiment requirement
1. 根据给出的Linux C代码，使用Rust语言进行重构

2. 将编码的Rust驱动代码通过Qemu运行起来，并且符合上面 test the module 的结果

3. 结果评判标准，在符合test要求的情况下，会从对rust的抽象、unsafe、bindings的使用程度进行分析(即尽量使用rust for linux提供的接口)

4. 项目小试验的截止时间为 4月 8号

### ¶ The end