//!
//! Translates a log or event call.
//!

use inkwell::values::BasicValue;

use crate::context::address_space::AddressSpace;
use crate::context::function::intrinsic::Intrinsic as IntrinsicFunction;
use crate::context::Context;
use crate::Dependency;

///
/// Translates a log or event call.
///
/// There are several cases of the translation for the sake of efficiency, since the front-end
/// emits topics and values sequentially by one, but the LLVM intrinsic and bytecode instruction
/// accept two at once.
///
pub fn log<'ctx, D>(
    context: &mut Context<'ctx, D>,
    range_start: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
    topics: Vec<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: Dependency,
{
    if topics.len() % 2 != 0 {
        topic_odd_number(context, range_start, length, topics)?;
        return Ok(None);
    }

    let data_empty_block = context.append_basic_block("event_even_data_empty");
    let data_not_empty_block = context.append_basic_block("event_even_data_not_empty");
    let join_block = context.append_basic_block("event_even_data_join");

    let data_empty_condition = context.builder().build_int_compare(
        inkwell::IntPredicate::EQ,
        length,
        context.field_const(0),
        "event_even_data_empty_condition",
    );
    context.build_conditional_branch(data_empty_condition, data_empty_block, data_not_empty_block);

    context.set_basic_block(data_empty_block);
    topic_even_number_data_empty(context, topics.clone())?;
    context.build_unconditional_branch(join_block);

    context.set_basic_block(data_not_empty_block);
    topic_even_number_data_not_empty(context, range_start, length, topics)?;
    context.build_unconditional_branch(join_block);

    context.set_basic_block(join_block);
    Ok(None)
}

///
/// Handles the even number of topics and empty data.
///
pub fn topic_even_number_data_empty<'ctx, D>(
    context: &mut Context<'ctx, D>,
    topics: Vec<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<()>
where
    D: Dependency,
{
    let event_initializer = context.field_const(topics.len() as u64);

    if topics.is_empty() {
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                event_initializer.as_basic_value_enum(),
                context.field_const(0).as_basic_value_enum(),
                context.field_const(1).as_basic_value_enum(),
            ],
            "event_even_data_empty_init_with_no_topics",
        );
        return Ok(());
    }

    let mut topic_index = 0;
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::Event),
        &[
            event_initializer.as_basic_value_enum(),
            topics[topic_index].as_basic_value_enum(),
            context.field_const(1).as_basic_value_enum(),
        ],
        "event_even_data_empty_init_with_first_topic",
    );
    topic_index += 1;
    while topics.len() - topic_index >= 2 {
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                topics[topic_index].as_basic_value_enum(),
                topics[topic_index + 1].as_basic_value_enum(),
                context.field_const(0).as_basic_value_enum(),
            ],
            "event_even_data_empty_next_two_topics",
        );
        topic_index += 2;
    }
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::Event),
        &[
            topics[topic_index].as_basic_value_enum(),
            context.field_const(0).as_basic_value_enum(),
            context.field_const(0).as_basic_value_enum(),
        ],
        "event_even_data_empty_last_topic",
    );

    Ok(())
}

///
/// Handles the even number of topics and non-empty data.
///
pub fn topic_even_number_data_not_empty<'ctx, D>(
    context: &mut Context<'ctx, D>,
    range_start: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
    topics: Vec<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<()>
where
    D: Dependency,
{
    let topics_length = context.field_const(topics.len() as u64);
    let data_length_shifted = context.builder().build_left_shift(
        length,
        context.field_const((compiler_common::BITLENGTH_X32) as u64),
        "event_even_data_length_shifted",
    );
    let event_initializer = context.builder().build_int_add(
        topics_length,
        data_length_shifted,
        "event_even_initializer",
    );

    let pointer = context.access_memory(
        range_start,
        AddressSpace::Heap,
        "event_even_first_value_pointer",
    );
    let value = context.build_load(pointer, "event_even_first_value");
    if topics.is_empty() {
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                event_initializer.as_basic_value_enum(),
                value,
                context.field_const(1).as_basic_value_enum(),
            ],
            "event_even_data_not_empty_init_with_value",
        );
    } else {
        let mut topic_index = 0;
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                event_initializer.as_basic_value_enum(),
                topics[topic_index].as_basic_value_enum(),
                context.field_const(1).as_basic_value_enum(),
            ],
            "event_even_data_not_empty_init_with_topic",
        );
        topic_index += 1;
        while topics.len() - topic_index >= 2 {
            context.build_call(
                context.get_intrinsic_function(IntrinsicFunction::Event),
                &[
                    topics[topic_index].as_basic_value_enum(),
                    topics[topic_index + 1].as_basic_value_enum(),
                    context.field_const(0).as_basic_value_enum(),
                ],
                "event_even_data_not_empty_next_two_topics",
            );
            topic_index += 2;
        }
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                topics[topic_index].as_basic_value_enum(),
                value,
                context.field_const(0).as_basic_value_enum(),
            ],
            "event_even_data_not_empty_last_topic",
        );
    }

    let range_start = context.builder().build_int_add(
        range_start,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "event_even_range_start_after_first",
    );
    let length = context.builder().build_int_sub(
        length,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "event_even_length_without_first",
    );
    data_loop(context, range_start, length)?;

    Ok(())
}

///
/// Handles the odd number of topics.
///
pub fn topic_odd_number<'ctx, D>(
    context: &mut Context<'ctx, D>,
    range_start: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
    topics: Vec<inkwell::values::IntValue<'ctx>>,
) -> anyhow::Result<()>
where
    D: Dependency,
{
    let topics_length = context.field_const(topics.len() as u64);
    let data_length_shifted = context.builder().build_left_shift(
        length,
        context.field_const((compiler_common::BITLENGTH_X32) as u64),
        "event_odd_data_length_shifted",
    );
    let event_initializer = context.builder().build_int_add(
        topics_length,
        data_length_shifted,
        "event_odd_initializer",
    );

    let mut topic_index = 0;
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::Event),
        &[
            event_initializer.as_basic_value_enum(),
            topics[topic_index].as_basic_value_enum(),
            context.field_const(1).as_basic_value_enum(),
        ],
        "event_odd_init_with_topic",
    );
    topic_index += 1;
    while topics.len() - topic_index >= 2 {
        context.build_call(
            context.get_intrinsic_function(IntrinsicFunction::Event),
            &[
                topics[topic_index].as_basic_value_enum(),
                topics[topic_index + 1].as_basic_value_enum(),
                context.field_const(0).as_basic_value_enum(),
            ],
            "event_odd_next_two_topics",
        );
        topic_index += 2;
    }
    data_loop(context, range_start, length)?;

    Ok(())
}

///
/// Generates the data writing loop.
///
pub fn data_loop<'ctx, D>(
    context: &mut Context<'ctx, D>,
    range_start: inkwell::values::IntValue<'ctx>,
    length: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<()>
where
    D: Dependency,
{
    let condition_block = context.append_basic_block("event_loop_condition");
    let body_block = context.append_basic_block("event_loop_body");
    let increment_block = context.append_basic_block("event_loop_increment");
    let join_block = context.append_basic_block("event_loop_join");

    let index_pointer = context.build_alloca(context.field_type(), "event_loop_index_pointer");
    let range_end = context
        .builder()
        .build_int_add(range_start, length, "event_loop_range_end");
    context.build_store(index_pointer, range_start);
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(condition_block);
    let index_value = context
        .build_load(index_pointer, "event_loop_index_value")
        .into_int_value();
    let condition = context.builder().build_int_compare(
        inkwell::IntPredicate::ULT,
        index_value,
        range_end,
        "event_loop_condition",
    );
    context.build_conditional_branch(condition, body_block, join_block);

    context.set_basic_block(increment_block);
    let index_value = context
        .build_load(index_pointer, "event_loop_index_value_increment")
        .into_int_value();
    let incremented = context.builder().build_int_add(
        index_value,
        context.field_const((compiler_common::SIZE_FIELD * 2) as u64),
        "event_loop_index_value_incremented",
    );
    context.build_store(index_pointer, incremented);
    context.build_unconditional_branch(condition_block);

    context.set_basic_block(body_block);
    let two_values_block = context.append_basic_block("event_loop_body_two_values");
    let one_value_block = context.append_basic_block("event_loop_body_one_value");
    let index_value = context
        .build_load(index_pointer, "event_loop_body_index_value")
        .into_int_value();
    let values_remaining =
        context
            .builder()
            .build_int_sub(range_end, index_value, "event_loop_values_remaining");
    let has_two_values = context.builder().build_int_compare(
        inkwell::IntPredicate::UGE,
        values_remaining,
        context.field_const((compiler_common::SIZE_FIELD * 2) as u64),
        "event_loop_has_two_values",
    );
    context.build_conditional_branch(has_two_values, two_values_block, one_value_block);

    context.set_basic_block(two_values_block);
    let value_1_pointer = context.access_memory(
        index_value,
        AddressSpace::Heap,
        "event_loop_value_1_pointer",
    );
    let value_1 = context.build_load(value_1_pointer, "event_loop_value_1");
    let index_value_next = context.builder().build_int_add(
        index_value,
        context.field_const(compiler_common::SIZE_FIELD as u64),
        "event_loop_index_value_next",
    );
    let value_2_pointer = context.access_memory(
        index_value_next,
        AddressSpace::Heap,
        "event_loop_value_2_pointer",
    );
    let value_2 = context.build_load(value_2_pointer, "event_loop_value_2");
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::Event),
        &[
            value_1,
            value_2,
            context.field_const(0).as_basic_value_enum(),
        ],
        "event_loop_call_with_two_values",
    );
    context.build_unconditional_branch(increment_block);

    context.set_basic_block(one_value_block);
    let value_1_pointer = context.access_memory(
        index_value,
        AddressSpace::Heap,
        "event_loop_value_1_pointer",
    );
    let value_1 = context.build_load(value_1_pointer, "event_loop_value_1");
    context.build_call(
        context.get_intrinsic_function(IntrinsicFunction::Event),
        &[
            value_1,
            context.field_const(0).as_basic_value_enum(),
            context.field_const(0).as_basic_value_enum(),
        ],
        "event_loop_call_with_value_and_zero",
    );
    context.build_unconditional_branch(increment_block);

    context.set_basic_block(join_block);
    Ok(())
}
