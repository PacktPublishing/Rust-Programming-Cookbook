extern crate tch;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::time::Instant;
use tch::{nn, nn::ModuleT, nn::OptimizerConfig, Device, Tensor};

#[derive(Debug)]
struct ConvNet {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl ConvNet {
    fn new(vs: &nn::Path, labels: i64) -> ConvNet {
        ConvNet {
            conv1: nn::conv2d(vs, 1, 32, 5, Default::default()),
            conv2: nn::conv2d(vs, 32, 64, 5, Default::default()),
            fc1: nn::linear(vs, 1024, 512, Default::default()),
            fc2: nn::linear(vs, 512, labels, Default::default()),
        }
    }
}

impl nn::ModuleT for ConvNet {
    fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
        xs.view([-1, 1, 28, 28])
            .apply(&self.conv1)
            .relu()
            .max_pool2d_default(2)
            .apply(&self.conv2)
            .relu()
            .max_pool2d_default(2)
            .view([-1, 1024]) // flatten
            .apply(&self.fc1)
            .relu()
            .dropout_(0.5, train)
            .apply(&self.fc2)
    }
}

fn train_from_scratch(learning_rate: f64, batch_size: i64, epochs: usize) -> failure::Fallible<()> {
    let data_path = Path::new("fashion-mnist/data/fashion");
    let model_path = Path::new("models/best.ot");

    if !data_path.exists() {
        println!(
            "Data not found at '{}'. Did you run 'git submodule update --init'?",
            data_path.to_string_lossy()
        );
        return Err(Error::from(ErrorKind::NotFound).into());
    }

    println!("Loading data from '{}'", data_path.to_string_lossy());
    let m = tch::vision::mnist::load_dir(data_path)?;

    let vs = nn::VarStore::new(Device::cuda_if_available());
    let net = ConvNet::new(&vs.root(), 10);
    let opt = nn::Adam::default().build(&vs, learning_rate)?;

    println!(
        "Starting training, saving model to '{}'",
        model_path.to_string_lossy()
    );

    let mut min_loss = ::std::f32::INFINITY;
    for epoch in 1..=epochs {
        let start = Instant::now();

        let mut losses = vec![];

        // Batched training, otherwise we would run out of memory
        for (image_batch, label_batch) in m.train_iter(batch_size).shuffle().to_device(vs.device())
        {
            let loss = net
                .forward_t(&image_batch, true)
                .cross_entropy_for_logits(&label_batch);
            opt.backward_step(&loss);

            losses.push(f32::from(loss));
        }
        let total_loss = losses.iter().sum::<f32>() / (losses.len() as f32);

        // Predict the test set without using batches
        let test_accuracy = net
            .forward_t(&m.test_images, false)
            .accuracy_for_logits(&m.test_labels);

        // Checkpoint
        if total_loss <= min_loss {
            vs.save(model_path)?;
            min_loss = total_loss;
        }

        // Output for the user
        println!(
            "{:4} | train loss: {:7.4} | test acc: {:5.2}% | duration: {}s",
            epoch,
            &total_loss,
            100. * f64::from(&test_accuracy),
            start.elapsed().as_secs()
        );
    }
    println!(
        "Done! The best model was saved to '{}'",
        model_path.to_string_lossy()
    );
    Ok(())
}

fn predict_from_best() -> failure::Fallible<()> {
    let data_path = Path::new("fashion-mnist/data/fashion");
    let model_weights_path = Path::new("models/best.ot");

    let m = tch::vision::mnist::load_dir(data_path)?;
    let mut vs = nn::VarStore::new(Device::cuda_if_available());
    let net = ConvNet::new(&vs.root(), 10);

    // restore weights
    println!("Loading model weights from '{}'", model_weights_path.to_string_lossy());
    vs.load(model_weights_path)?;

    println!("Probabilities and predictions for 10 random images in the test set");
    for (image_batch, label_batch) in m.test_iter(1).shuffle().to_device(vs.device()).take(10) {
        let raw_tensor = net
            .forward_t(&image_batch, false)
            .softmax(-1).view(m.labels);
        let predicted_index: Vec<i64> = raw_tensor.argmax(0, false).into();
        let probabilities: Vec<f64> = raw_tensor.into();

        print!("[ ");
        for p in probabilities {
            print!("{:.4} ", p);
        }
        let label: Vec<i64> = label_batch.into(); 
        println!("] predicted {}, was {}", predicted_index[0], label[0]);
    }
    Ok(())
}

fn main() -> failure::Fallible<()> {
    train_from_scratch(1e-2, 1024, 5)?;
    predict_from_best()?;
    Ok(())
}
