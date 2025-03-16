use bytemuck::{Pod, Zeroable};
use std::marker::PhantomData;
use std::ops::Range;
use wgpu::util::DeviceExt;

/// Wrapper around wgpu::Buffer with additional metadata and helper methods
#[derive(Debug)]
pub struct Buffer<T: Pod + Zeroable = u8> {
    /// The underlying wgpu buffer
    pub buffer: wgpu::Buffer,
    /// Number of elements in the buffer
    pub length: usize,
    /// Size of the buffer in bytes
    pub size: u64,
    /// Usage flags for the buffer
    pub usage: wgpu::BufferUsages,
    /// Phantom data to keep track of the generic type
    _marker: PhantomData<T>,
}

/// Builder for creating buffers
pub struct BufferBuilder<'a, T: Pod + Zeroable = u8> {
    /// The device used to create the buffer
    device: &'a wgpu::Device,
    /// The queue used to write to the buffer (optional)
    queue: Option<&'a wgpu::Queue>,
    /// The data to initialize the buffer with
    contents: Option<&'a [T]>,
    /// The size of the buffer in elements (used if contents is None)
    size: Option<usize>,
    /// The usage flags for the buffer
    usage: wgpu::BufferUsages,
    /// The label for the buffer
    label: String,
    /// Whether the buffer should be mapped at creation
    mapped_at_creation: bool,
}

impl<'a, T: Pod + Zeroable> BufferBuilder<'a, T> {
    /// Creates a new buffer builder
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            device,
            queue: None,
            contents: None,
            size: None,
            usage: wgpu::BufferUsages::empty(),
            label: "buffer".to_string(),
            mapped_at_creation: false,
        }
    }

    /// Sets the queue to use for buffer updates
    pub fn queue(mut self, queue: &'a wgpu::Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    /// Sets the contents to initialize the buffer with
    pub fn contents(mut self, contents: &'a [T]) -> Self {
        self.contents = Some(contents);
        self
    }

    /// Sets the size of the buffer in elements (used if contents is None)
    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Adds usage flags to the buffer
    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.usage |= usage;
        self
    }

    /// Sets the label for the buffer
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets whether the buffer should be mapped at creation
    pub fn mapped_at_creation(mut self, mapped: bool) -> Self {
        self.mapped_at_creation = mapped;
        self
    }

    /// Builds the buffer
    pub fn build(self) -> Result<Buffer<T>, String> {
        // Ensure we have either contents or size
        if self.contents.is_none() && self.size.is_none() {
            return Err("Either contents or size must be provided".to_string());
        }

        // Ensure we have some usage flags
        if self.usage.is_empty() {
            return Err("Buffer must have at least one usage flag".to_string());
        }

        let contents = self.contents;
        let size = match (contents, self.size) {
            (Some(data), _) => data.len(),
            (None, Some(size)) => size,
            _ => unreachable!(), // We already checked above
        };

        let byte_size = (size * std::mem::size_of::<T>()) as u64;

        // Create the buffer
        let buffer = if let Some(data) = contents {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&self.label),
                    contents: bytemuck::cast_slice(data),
                    usage: self.usage,
                })
        } else {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&self.label),
                size: byte_size,
                usage: self.usage,
                mapped_at_creation: self.mapped_at_creation,
            })
        };

        Ok(Buffer {
            buffer,
            length: size,
            size: byte_size,
            usage: self.usage,
            _marker: PhantomData,
        })
    }
}

impl<T: Pod + Zeroable> Buffer<T> {
    /// Creates a new buffer with the given contents
    pub fn new(
        device: &wgpu::Device,
        contents: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> Self {
        BufferBuilder::new(device)
            .contents(contents)
            .usage(usage)
            .label(label)
            .build()
            .expect("Failed to create buffer")
    }

    /// Updates the buffer with new data
    pub fn update(&self, queue: &wgpu::Queue, data: &[T], offset: usize) {
        let offset_bytes = (offset * std::mem::size_of::<T>()) as u64;
        queue.write_buffer(&self.buffer, offset_bytes, bytemuck::cast_slice(data));
    }

    /// Updates the entire buffer with new data
    pub fn update_all(&self, queue: &wgpu::Queue, data: &[T]) {
        self.update(queue, data, 0);
    }

    /// Creates a binding resource for the entire buffer
    pub fn as_entire_binding(&self) -> wgpu::BindingResource {
        self.buffer.as_entire_binding()
    }

    /// Gets the slice of the entire buffer
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.slice(..)
    }

    /// Gets a slice of the buffer with the given range
    pub fn slice_range(&self, range: Range<u64>) -> wgpu::BufferSlice {
        self.buffer.slice(range)
    }
}

/// Implementation of DynamicBuffer for updating data frequently
pub struct DynamicBuffer<T: Pod + Zeroable> {
    /// Current buffer to read from
    pub read_buffer: Buffer<T>,
    /// Current buffer to write to
    pub write_buffer: Buffer<T>,
    /// Usage flags for the buffers
    usage: wgpu::BufferUsages,
    /// Label for the buffers
    label: String,
}

impl<T: Pod + Zeroable> DynamicBuffer<T> {
    /// Creates a new dynamic buffer with the given initial data
    pub fn new(
        device: &wgpu::Device,
        initial_data: &[T],
        usage: wgpu::BufferUsages,
        label: &str,
    ) -> Self {
        let read_buffer = Buffer::new(device, initial_data, usage, &format!("{}_read", label));
        let write_buffer = Buffer::new(device, initial_data, usage, &format!("{}_write", label));

        Self {
            read_buffer,
            write_buffer,
            usage,
            label: label.to_string(),
        }
    }

    /// Updates the buffer, automatically resizing if needed
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
        if data.len() > self.write_buffer.length {
            // Need to resize
            self.read_buffer =
                Buffer::new(device, data, self.usage, &format!("{}_read", self.label));
            self.write_buffer =
                Buffer::new(device, data, self.usage, &format!("{}_write", self.label));
        } else {
            // Just update the write buffer
            self.write_buffer.update_all(queue, data);
            // Swap the buffers
            std::mem::swap(&mut self.read_buffer, &mut self.write_buffer);
        }
    }

    /// Gets the current size of the buffer in elements
    pub fn len(&self) -> usize {
        self.read_buffer.length
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Ensures the buffer has at least the specified capacity
    /// note: min_capacity is in bytes
    pub fn ensure_capacity(&mut self, device: &wgpu::Device, min_capacity: usize) {
        if min_capacity > self.read_buffer.length {
            self.read_buffer = BufferBuilder::new(device)
                .size(min_capacity)
                .usage(self.usage)
                .label(format!("{}_read", self.label))
                .build()
                .expect("Failed to create resized buffer");

            self.write_buffer = BufferBuilder::new(device)
                .size(min_capacity)
                .usage(self.usage)
                .label(format!("{}_write", self.label))
                .build()
                .expect("Failed to create resized buffer");
        }
    }
}
